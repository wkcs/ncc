use std::collections::HashMap;
use std::process;

#[derive(Debug)]
pub enum CmdValType {
    NoVal,
    OptVal,
    ValSpace,
    ValNoSpace,
    ValOptSpace,
}

#[derive(Debug)]
pub struct CmdInfo {
    pub short: String,
    pub long: String,
    pub help: String,
    pub val_type: CmdValType,
    pub val_str: String,
    pub index: usize,
}

#[derive(Debug)]
pub enum CmdMatchType {
    Short,
    Long,
}

impl CmdInfo {
    pub fn eq(&self, other: &Self) -> bool {
        (self.short == other.short) && (self.long == other.long)
    }
}

#[derive(Debug)]
pub struct CmdLine {
    pub info: Vec<CmdInfo>,
    pub args: HashMap<usize, Vec<String>>,
    pub others: Vec<String>,
}

impl CmdLine {
    pub fn new() -> Self {
        let cmdline = CmdLine {
            info: Vec::<CmdInfo>::new(),
            args: HashMap::<usize, Vec<String>>::new(),
            others: Vec::<String>::new(),
        };

        return cmdline;
    }

    pub fn add(
        &mut self,
        short: &str,
        long: &str,
        help: &str,
        val_type: CmdValType,
        val_str: &str,
    ) {
        let cmd = CmdInfo {
            short: String::from(short),
            long: String::from(long),
            help: String::from(help),
            val_type: val_type,
            val_str: if val_str.len() == 0 {
                String::from("value")
            } else {
                String::from(val_str)
            },
            index: self.info.len(),
        };

        if (short.len() == 0) && (long.len() == 0) {
            eprintln!("Missing command.");
            return;
        }
        if help.len() == 0 {
            eprintln!("{} {}: Missing help info.", short, long);
            return;
        }

        for tmp in &self.info {
            if tmp.eq(&cmd) {
                return;
            }
        }

        self.info.push(cmd);
    }

    pub fn help(&self) -> String {
        let mut help = String::new();

        for cmd in &self.info {
            let mut tmp_str = String::from("  ");

            if cmd.short.len() > 0 {
                tmp_str += &cmd.short;
                if cmd.long.len() > 0 {
                    tmp_str += format!("({})", cmd.long).as_str();
                }
            } else {
                tmp_str += &cmd.long[..];
            }

            match cmd.val_type {
                CmdValType::OptVal => tmp_str += format!("[{}]", cmd.val_str).as_str(),
                CmdValType::ValSpace => tmp_str += format!(" <{}>", cmd.val_str).as_str(),
                CmdValType::ValNoSpace => tmp_str += format!("<{}>", cmd.val_str).as_str(),
                CmdValType::ValOptSpace => tmp_str += format!("[ ]<{}>", cmd.val_str).as_str(),
                _ => (),
            }

            if tmp_str.len() < 24 {
                for _ in tmp_str.len()..25 {
                    tmp_str.push(' ');
                }
            } else {
                tmp_str.push('\n');
                for _ in 0..25 {
                    tmp_str.push(' ');
                }
            }
            tmp_str += &cmd.help;
            tmp_str.push('\n');

            help += &tmp_str;
        }
        help.pop();

        return help;
    }

    pub fn get_index(&self, str: &str) -> Option<usize> {
        for cmd in &self.info {
            if (cmd.long == str) || (cmd.short == str) {
                return Some(cmd.index);
            }
        }

        return None;
    }

    pub fn cmd_is_meatch(cmd: &CmdInfo, str: &str) -> Option<CmdMatchType> {
        match cmd.val_type {
            CmdValType::NoVal | CmdValType::ValSpace => {
                if cmd.short == str {
                    return Some(CmdMatchType::Short);
                }
                if cmd.long == str {
                    return Some(CmdMatchType::Long);
                } else {
                    return None;
                }
            }
            CmdValType::OptVal | CmdValType::ValNoSpace | CmdValType::ValOptSpace => {
                if cmd.long.len() > 0 {
                    let len = if cmd.long.len() < str.len() {
                        cmd.long.len()
                    } else {
                        str.len()
                    };
                    if cmd.long[..len] == str[..len] {
                        return Some(CmdMatchType::Long);
                    }
                }

                if cmd.short.len() > 0 {
                    let len = if cmd.short.len() < str.len() {
                        cmd.short.len()
                    } else {
                        str.len()
                    };
                    if cmd.short[..len] == str[..len] {
                        return Some(CmdMatchType::Short);
                    }
                }

                return None;
            }
        }
    }

    pub fn parse(&mut self, args: &Vec<String>) {
        let mut get_valne = false;
        let mut index: usize = 0;
        let mut meatched = false;

        for arg in args {
            if get_valne {
                get_valne = false;
                let arg_tmp = self.args.get(&index);
                if let Some(arg_val) = arg_tmp {
                    let mut val_tmp: Vec<String> = arg_val.to_vec();
                    val_tmp.push(arg.to_string());
                    self.args.insert(index, val_tmp);
                } else {
                    self.args.insert(index, vec![arg.to_string()]);
                }
            } else {
                for cmd in &self.info {
                    if let Some(match_type) = Self::cmd_is_meatch(cmd, arg) {
                        meatched = true;
                        match cmd.val_type {
                            CmdValType::NoVal => {
                                self.args.insert(cmd.index, Vec::<String>::new());
                            }
                            CmdValType::ValSpace => {
                                index = cmd.index;
                                get_valne = true;
                            }
                            CmdValType::OptVal => {
                                let mut value = String::new();
                                match match_type {
                                    CmdMatchType::Long => {
                                        if cmd.long.len() < arg.len() {
                                            value = arg[cmd.long.len()..].to_string();
                                        }
                                    }
                                    CmdMatchType::Short => {
                                        if cmd.short.len() < arg.len() {
                                            value = arg[cmd.short.len()..].to_string();
                                        }
                                    }
                                }

                                let arg_tmp = self.args.get(&cmd.index);
                                if let Some(arg_val) = arg_tmp {
                                    if value.len() > 0 {
                                        let mut val_tmp: Vec<String> = arg_val.to_vec();
                                        val_tmp.push(value);
                                        self.args.insert(cmd.index, val_tmp);
                                    }
                                } else {
                                    if value.len() > 0 {
                                        self.args.insert(cmd.index, vec![value]);
                                    } else {
                                        self.args.insert(cmd.index, Vec::<String>::new());
                                    }
                                }
                            }
                            CmdValType::ValNoSpace => {
                                let value: String;
                                match match_type {
                                    CmdMatchType::Long => {
                                        if cmd.long.len() < arg.len() {
                                            value = arg[cmd.long.len()..].to_string();
                                        } else {
                                            process::exit(-1);
                                        }
                                    }
                                    CmdMatchType::Short => {
                                        if cmd.short.len() < arg.len() {
                                            value = arg[cmd.short.len()..].to_string();
                                        } else {
                                            process::exit(-1);
                                        }
                                    }
                                }

                                let arg_tmp = self.args.get(&cmd.index);
                                if let Some(arg_val) = arg_tmp {
                                    let mut val_tmp: Vec<String> = arg_val.to_vec();
                                    val_tmp.push(value);
                                    self.args.insert(cmd.index, val_tmp);
                                } else {
                                    self.args.insert(cmd.index, vec![value]);
                                }
                            }
                            CmdValType::ValOptSpace => {
                                let mut value = String::new();
                                match match_type {
                                    CmdMatchType::Long => {
                                        if cmd.long.len() < arg.len() {
                                            value = arg[cmd.long.len()..].to_string();
                                        } else {
                                            index = cmd.index;
                                            get_valne = true;
                                            break;
                                        }
                                    }
                                    CmdMatchType::Short => {
                                        if cmd.short.len() < arg.len() {
                                            value = arg[cmd.short.len()..].to_string();
                                        } else {
                                            index = cmd.index;
                                            get_valne = true;
                                            break;
                                        }
                                    }
                                }

                                let arg_tmp = self.args.get(&cmd.index);
                                if let Some(arg_val) = arg_tmp {
                                    let mut val_tmp: Vec<String> = arg_val.to_vec();
                                    val_tmp.push(value);
                                    self.args.insert(cmd.index, val_tmp);
                                } else {
                                    self.args.insert(cmd.index, vec![value]);
                                }
                            }
                        }
                        break;
                    }
                }
                if !meatched {
                    self.others.push(arg.to_string());
                }
                meatched = false;
            }
        }

        if get_valne {
            eprintln!(
                "{}: Missing value.",
                if self.info[index].short.len() > 0 {
                    &self.info[index].short
                } else {
                    &self.info[index].long
                }
            );
            process::exit(-1);
        }
    }

    pub fn is_include(&self, str: &str) -> bool {
        if let Some(index) = self.get_index(str) {
            if let Some(_) = self.args.get(&index) {
                return true;
            }
        }

        return false;
    }

    pub fn get_value(&self, cmd: &CmdInfo) -> Option<&Vec<String>> {
        if let Some(vals) = self.args.get(&cmd.index) {
            return Some(vals);
        }

        return None;
    }

    pub fn get_value_by_name(&self, str: &str) -> Option<&Vec<String>> {
        if let Some(index) = self.get_index(str) {
            if let Some(vals) = self.args.get(&index) {
                return Some(vals);
            }
        }

        return None;
    }

    pub fn get_value_by_index(&self, index: usize) -> Option<&Vec<String>> {
        if let Some(vals) = self.args.get(&index) {
            return Some(vals);
        }

        return None;
    }
}
