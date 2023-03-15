#![allow(unused)]

extern crate clap;
use clap::Parser;
use std::process::Command;

#[derive(Debug, Clone, Copy)]
struct Mon<T> {
    id: T,
    desc: T,
}

#[derive(Clone)]
struct MonCmd<T> {
    desc: T,
    cmds: Vec<Vec<T>>,
}

impl MonCmd<String> {
    fn replace_moninfo(&self, mon: Mon<String>) -> Vec<Vec<String>> {
        self.cmds
            .iter()
            .map(|c| {
                c.iter()
                    .map(|x| {
                        if x.contains("&(") {
                            let mut finstring: String = "".to_string();
                            if x.contains("&(id)") {
                                finstring = x.replace("&(id)", &mon.id);
                            } else if x.contains("&(desc)") {
                                finstring = x.replace("&(desc)", &mon.desc);
                            }
                            finstring
                        } else {
                            x.to_string()
                        }
                    })
                    .collect()
            })
            .collect()
    }
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    monitor: String,

    #[arg(default_value_t = true, short, long)]
    keyword: bool,

    #[arg(default_value_t = false, short, long)]
    dispatcher: bool,

    #[arg(short, long)]
    command: String,
}

fn main() {
    let args = Args::parse();

    // let mut moncmds: Vec<MonCmd<String>> = vec![MonCmd {
    //     desc: args.monitor,
    //     cmds: vec![vec![args.command.split(" ")]],
    // }];

    let cmd: MonCmd<String> = MonCmd {
        desc: args.monitor.to_owned(),
        cmds: vec![args.command.split(' ').map(|x| x.to_owned()).collect()],
    };

    let output = Command::new("/usr/bin/hyprctl")
        .arg("monitors")
        .output()
        .unwrap();

    let file = String::from_utf8(output.stdout).unwrap();

    let mut mons: Vec<Mon<String>> = vec![];

    file.lines().for_each(|y| {
        if y.contains("description") {
            let st = y.to_string();
            let s = st.find('(').unwrap() + 1;
            let ds = st.find(':').unwrap() + 2;
            let i = st.get(s..(st.len() - 1)).unwrap().to_owned();
            let d = st.get(ds..(s - 2)).unwrap().to_owned();
            println!("mon:{}, desc:{}", i, d);
            mons.push(Mon { id: i, desc: d });
        }
    });

    mons.iter().for_each(|x| {
        // moncmds.iter().for_each(|y| {
        if x.desc.contains(&cmd.desc) {
            let repmon = cmd.replace_moninfo(x.to_owned());
            repmon.iter().for_each(|z| {
                let typ = |disp: bool| {
                    if (disp) {
                        return "dispatch";
                    }
                    "keyword"
                };
                let dispat = args.dispatcher;
                println!("/usr/bin/hyprctl {} {}", typ(dispat), z.join(" "));
                let arg2 = &z[1..].join(" ");
                let rtrn = Command::new("/usr/bin/hyprctl")
                    .arg(typ(dispat))
                    .arg(&z[0])
                    .arg(arg2)
                    .output()
                    .unwrap();
                println!("{}", std::str::from_utf8(&rtrn.stderr).unwrap());
                println!("{}", std::str::from_utf8(&rtrn.stdout).unwrap());
            });
        }
        // })
    });

    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            println!("Ok.");
        } else {
            eprintln!("Failed.");
        }
    } else {
        eprintln!("Interrupted!");
    }
}
