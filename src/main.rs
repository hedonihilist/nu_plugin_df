mod mountinfo;

use crate::mountinfo::get_mountinfo_list;
use mountinfo::MountInfo;
use nix::sys::statvfs::Statvfs;
use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{
    CallInfo, Dictionary, ReturnSuccess, ReturnValue, Signature, UntaggedValue, Value,
};
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
pub struct Options {
    pub show_local_fs: bool,
    pub show_all_fs: bool,
    pub listed_fs: HashSet<String>,
    pub excluded_fs: HashSet<String>,
    pub human_readable: bool,
    pub print_grand_total: bool,
    pub field_list: Vec<String>,
    pub human_readable_1024: bool, // true => show size in powers of 1024, false => powers of 1000
    pub inodes: bool,
    pub show_fs_type: bool,
    pub output_all_fields: bool,
}

impl Options {
    pub fn new() -> Options {
        Options {
            show_local_fs: true,
            show_all_fs: false,
            human_readable: true,
            ..Default::default()
        }
    }
}

struct Df;

impl Plugin for Df {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(
            Signature::build("df")
                .switch(
                    "all", "Show all file systems, including include pseudo, duplicate, inaccessible file systems", Some('a')
                )
                .switch(
                    "local", "Show local file systems only.", Some('l')).filter()
        )
    }

    fn begin_filter(&mut self, call_info: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        let mut options = Options::new();
        if let Some(args) = call_info.args.named {
            options.show_all_fs = args.get("all").is_some();
            options.show_local_fs = args.get("local").is_some();
        }
        let value_list = get_all_entries(&options);
        Ok(value_list
            .into_iter()
            .map(|v| ReturnSuccess::value(v))
            .collect())
    }

    fn filter(&mut self, _input: Value) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(vec![])
    }
}

fn get_all_entries(options: &Options) -> Vec<UntaggedValue> {
    let mut rows: Vec<UntaggedValue> = vec![];

    let mountinfo_list = filter_mountinfo_list(get_mountinfo_list(), &options);

    for mount in mountinfo_list.into_iter() {
        if let Some(stat) = get_dev(&mount, options) {
            let mut dict = Dictionary::default();

            dict.insert(
                "Filesystem".into(),
                UntaggedValue::from(mount.mount_source.to_string()).into(),
            );
            dict.insert(
                "Type".into(),
                UntaggedValue::from(mount.fs_type.to_string()).into(),
            );

            // inodes
            let itotal = stat.files();
            let ifree = stat.files_available();
            let iused = itotal - ifree;
            // cast to i64 to get proper alignment
            dict.insert("Inodes".into(), UntaggedValue::int(itotal as i64).into());
            dict.insert("IUsed".into(), UntaggedValue::int(iused as i64).into());
            dict.insert("IFree".into(), UntaggedValue::int(ifree as i64).into());
            let ipercent: f64 = match itotal != 0 {
                true => 100f64 * (iused as f64) / (ifree + iused) as f64,
                false => 0f64,
            };
            dict.insert("IUse%".into(), UntaggedValue::from(ipercent).into());

            // size
            // in B
            let total = stat.blocks() * stat.fragment_size();
            let free = stat.blocks_available() * stat.fragment_size();
            let used = total - free;
            dict.insert("Total".into(), UntaggedValue::filesize(total).into());
            dict.insert("Used".into(), UntaggedValue::filesize(used).into());
            dict.insert("Avail".into(), UntaggedValue::filesize(free).into());
            let percent: f64 = match total != 0 {
                true => 100f64 * (used as f64) / (free + used) as f64,
                false => 0f64,
            };
            dict.insert("Use%".into(), UntaggedValue::from(percent).into());
            dict.insert(
                "mount-point".into(),
                UntaggedValue::filepath(mount.mount_point.to_string()).into(),
            );
            rows.push(UntaggedValue::Row(dict));
        }
    }
    rows
}

fn get_dev(mount: &MountInfo, options: &Options) -> Option<Statvfs> {
    if mount.is_remote() && options.show_local_fs {
        return None;
    }
    if mount.is_dummy() && !options.show_all_fs && options.listed_fs.is_empty() {
        return None;
    }
    // fs_type not listed
    if !options.listed_fs.is_empty() && !options.listed_fs.contains(&mount.fs_type)
        || options.excluded_fs.contains(&mount.fs_type)
    {
        return None;
    }

    let res_stat = nix::sys::statvfs::statvfs::<str>(mount.mount_point.as_ref());

    if res_stat.is_err() {
        return None;
    }
    let stat = res_stat.unwrap();

    if stat.blocks() == 0 && !options.show_all_fs && options.listed_fs.is_empty() {
        return None;
    }

    Some(stat)
}

fn filter_mountinfo_list(list: Vec<MountInfo>, options: &Options) -> Vec<MountInfo> {
    let mut filtered: Vec<MountInfo> = vec![];
    let mut seen: HashMap<u64, usize> = HashMap::new();
    for me in list.into_iter() {
        let mut discard_me: Option<usize> = None; //
                                                  // skip
        if (me.is_remote() && options.show_local_fs)
            || (me.is_dummy() && !options.show_all_fs && !options.listed_fs.contains(&me.fs_type))
            || (!options.listed_fs.is_empty() && !options.listed_fs.contains(&me.fs_type))
            || options.excluded_fs.contains(&me.fs_type)
        {
            // pass
        } else {
            /*
            在Linux中有一个bind mount的概念，能把一个目录挂载到另一个目录下，例如mount -o bind /boot/efi /tmp/bindmount
            在df的输出中我们希望去除重复的设备
             */
            if let Some(&idx) = seen.get(&me.dev()) {
                let seen_dev: &MountInfo = &filtered[idx];

                // target指当前me
                // source指
                let target_nearer_root = seen_dev.mount_point.len() > me.mount_point.len();
                let source_below_root = !seen_dev.root.is_empty()
                    && !me.root.is_empty()
                    && (seen_dev.root.len() < me.root.len());
                if !options.print_grand_total
                    && me.is_remote()
                    && seen_dev.is_remote()
                    && seen_dev.mount_source.eq(&me.mount_source)
                {
                    // don't discard
                } else if (me.mount_source.contains('/') && !seen_dev.mount_source.contains('/'))
                    || (target_nearer_root && !source_below_root)
                    || (!seen_dev.mount_source.eq(&me.mount_source)
                        && seen_dev.mount_point.eq(&me.mount_point))
                {
                    // discard this one
                    continue;
                } else {
                    discard_me = Some(idx);
                }
            }
        }
        if let Some(discard_idx) = discard_me {
            filtered[discard_idx] = me;
            //std::mem::replace(&mut filtered[discard_idx], me);
        } else {
            let dev = me.dev();
            filtered.push(me);
            seen.insert(dev, filtered.len() - 1);
        }
    }
    filtered
}

fn main() {
    serve_plugin(&mut Df {});
}
