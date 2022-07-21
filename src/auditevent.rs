// Copyright (C) 2021, Achiefs.

// To implement Debug and fmt method
use std::fmt;
// To handle files
use std::fs::OpenOptions;
use std::io::Write;
// Handle time intervals
//use std::time::Duration;
// To log the program procedure
use log::*;
// To handle JSON objects
use serde_json::{json, to_string};
// To manage paths
//use std::path::PathBuf;
// To manage HTTP requests
//use reqwest::Client;

// To get configuration constants
use crate::config;

// ----------------------------------------------------------------------------

pub struct Event {
    pub id: String,
    pub timestamp: String,
    pub hostname: String,
    pub node: String,
    pub version: String,
    pub path: String,
    pub file: String,
    pub labels: Vec<String>,
    pub operation: String,
    pub checksum: String,
    pub fpid: u32,
    pub system: String,
    pub command: String,

    pub ogid: String,
    pub rdev: String,
    pub proctitle: String,
    pub cap_fver: String,
    pub inode: String,
    pub cap_fp: String,
    pub cap_fe: String,
    pub item: String,
    pub cap_fi: String,
    pub dev: String,
    pub mode: String,
    pub cap_frootid: String,
    pub ouid: String,
    pub parent_inode: String,
    pub parent_cap_fe: String,
    pub parent_cap_frootid: String,
    pub parent_ouid: String,
    pub parent_item: String,
    pub parent_cap_fver: String,
    pub parent_mode: String,
    pub parent_rdev: String,
    pub parent_cap_fi: String,
    pub parent_cap_fp: String,
    pub parent_dev: String,
    pub parent_ogid: String,
    pub cwd: String,
    pub syscall: String,
    pub ppid: String,
    pub comm: String,
    pub fsuid: String,
    pub pid: String,
    pub a0: String,
    pub a1: String,
    pub a2: String,
    pub a3: String,
    pub arch: String,
    pub auid: String,
    pub items: String,
    pub gid: String,
    pub euid: String,
    pub sgid: String,
    pub uid: String,
    pub tty: String,
    pub success: String,
    pub exit: String,
    pub ses: String,
    pub key: String,
    pub suid: String,
    pub egid: String,
    pub fsgid: String,
    pub exe: String,
    pub source: String,
}

impl Event {
    pub fn new() -> Self {
        Event {
            id: String::from(""),
            timestamp: String::from(""),
            hostname: String::from(""),
            node: String::from(""),
            version: String::from(config::VERSION),
            path: String::from(""),
            file: String::from(""),
            labels: Vec::<String>::new(),
            operation: String::from(""),
            checksum: String::from(""),
            fpid: 0,
            system: String::from(""),
            command: String::from(""),

            ogid: String::from(""),
            rdev: String::from(""),
            proctitle: String::from(""),
            cap_fver: String::from(""),
            inode: String::from(""),
            cap_fp: String::from(""),
            cap_fe: String::from(""),
            item: String::from(""),
            cap_fi: String::from(""),
            dev: String::from(""),
            mode: String::from(""),
            cap_frootid: String::from(""),
            ouid: String::from(""),
            parent_inode: String::from(""),
            parent_cap_fe: String::from(""),
            parent_cap_frootid: String::from(""),
            parent_ouid: String::from(""),
            parent_item: String::from(""),
            parent_cap_fver: String::from(""),
            parent_mode: String::from(""),
            parent_rdev: String::from(""),
            parent_cap_fi: String::from(""),
            parent_cap_fp: String::from(""),
            parent_dev: String::from(""),
            parent_ogid: String::from(""),
            cwd: String::from(""),
            syscall: String::from(""),
            ppid: String::from(""),
            comm: String::from(""),
            fsuid: String::from(""),
            pid: String::from(""),
            a0: String::from(""),
            a1: String::from(""),
            a2: String::from(""),
            a3: String::from(""),
            arch: String::from(""),
            auid: String::from(""),
            items: String::from(""),
            gid: String::from(""),
            euid: String::from(""),
            sgid: String::from(""),
            uid: String::from(""),
            tty: String::from(""),
            success: String::from(""),
            exit: String::from(""),
            ses: String::from(""),
            key: String::from(""),
            suid: String::from(""),
            egid: String::from(""),
            fsgid: String::from(""),
            exe: String::from(""),
            source: String::from("audit")
        }
    }

    // Get formatted string with all required data
    fn format_json(&self) -> String {
        let obj = json!({
            "id": self.id.clone(),
            "timestamp": self.timestamp.clone(),
            "hostname": self.hostname.clone(),
            "node": self.node.clone(),
            "version": self.version.clone(),
            "path": self.path.clone(),
            "file": self.file.clone(),
            "labels": self.labels.clone(),
            "operation": self.operation.clone(),
            "checksum": self.checksum.clone(),
            "fpid": self.fpid.clone(),
            "system": self.system.clone(),
            "command": self.command.clone(),

            "ogid": self.ogid.clone(),
            "rdev": self.rdev.clone(),
            "proctitle": self.proctitle.clone(),
            "cap_fver": self.cap_fver.clone(),
            "inode": self.inode.clone(),
            "cap_fp": self.cap_fp.clone(),
            "cap_fe": self.cap_fe.clone(),
            "item": self.item.clone(),
            "cap_fi": self.cap_fi.clone(),
            "dev": self.dev.clone(),
            "mode": self.mode.clone(),
            "cap_frootid": self.cap_frootid.clone(),
            "ouid": self.ouid.clone(),
            "parent_inode": self.parent_inode.clone(),
            "parent_cap_fe": self.parent_cap_fe.clone(),
            "parent_cap_frootid": self.parent_cap_frootid.clone(),
            "parent_ouid": self.parent_ouid.clone(),
            "parent_item": self.parent_item.clone(),
            "parent_cap_fver": self.parent_cap_fver.clone(),
            "parent_mode": self.parent_mode.clone(),
            "parent_rdev": self.parent_rdev.clone(),
            "parent_cap_fi": self.parent_cap_fi.clone(),
            "parent_cap_fp": self.parent_cap_fp.clone(),
            "parent_dev": self.parent_dev.clone(),
            "parent_ogid": self.parent_ogid.clone(),
            "cwd": self.cwd.clone(),
            "syscall": self.syscall.clone(),
            "ppid": self.ppid.clone(),
            "comm": self.comm.clone(),
            "fsuid": self.fsuid.clone(),
            "pid": self.pid.clone(),
            "a0": self.a0.clone(),
            "a1": self.a1.clone(),
            "a2": self.a2.clone(),
            "a3": self.a3.clone(),
            "arch": self.arch.clone(),
            "auid": self.auid.clone(),
            "items": self.items.clone(),
            "gid": self.gid.clone(),
            "euid": self.euid.clone(),
            "sgid": self.sgid.clone(),
            "uid": self.uid.clone(),
            "tty": self.tty.clone(),
            "success": self.success.clone(),
            "exit": self.exit.clone(),
            "ses": self.ses.clone(),
            "key": self.key.clone(),
            "suid": self.suid.clone(),
            "egid": self.egid.clone(),
            "fsgid": self.fsgid.clone(),
            "exe": self.exe.clone(),
            "source": self.source.clone()
        });
        to_string(&obj).unwrap()
    }

    // ------------------------------------------------------------------------

    // Function to write the received events to file
    pub fn log_event(&self, file: String){
        let mut events_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file)
            .expect("(auditevent::log_event) Unable to open events log file.");

            match writeln!(events_file, "{}", self.format_json()) {
                Ok(_d) => debug!("Written audit event Log"),
                Err(e) => error!("Audit event could not be written, Err: [{}]", e)
            };
    }

    // ------------------------------------------------------------------------

    // Function to send events through network
    /*pub async fn send(&self, index: String, address: String, user: String, pass: String, insecure: bool) {
        let data = json!({
            "timestamp": self.timestamp.clone(),
            "hostname": self.hostname.clone(),
            "node": self.node.clone(),
            "pid": self.pid.clone(),
            "version": self.version.clone(),
            "labels": self.labels.clone(),
            "operation": self.operation.clone(),
            "file": String::from(self.path.clone().to_str().unwrap()),
            "checksum": self.checksum.clone(),
            "system": self.system.clone()
        });

        let request_url = format!("{}/{}/_doc/{}", address, index, self.id);
        let client = Client::builder()
            .danger_accept_invalid_certs(insecure)
            .timeout(Duration::from_secs(30))
            .build().unwrap();
        match client
            .post(request_url)
            .basic_auth(user, Some(pass))
            .json(&data)
            .send()
            .await{
            Ok(response) => debug!("Response received: {:?}", response),
            Err(e) => debug!("Error on request: {:?}", e)
        };
    }*/
}

// ----------------------------------------------------------------------------

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        f.debug_struct("")
          .field("id", &self.id)
          .field("path", &self.path)
          .field("operation", &self.operation)
          .field("file", &self.file)
          .field("timestamp", &self.timestamp)
          .field("proctitle", &self.proctitle)
          .field("cap_fver", &self.cap_fver)
          .field("inode", &self.inode)
          .field("cap_fp", &self.cap_fp)
          .field("cap_fe", &self.cap_fe)
          .field("item", &self.item)
          .field("cap_fi", &self.cap_fi)
          .field("dev", &self.dev)
          .field("mode", &self.mode)
          .field("cap_frootid", &self.cap_frootid)
          .field("ouid", &self.ouid)
          .field("parent_inode", &self.parent_inode)
          .field("parent_cap_fe", &self.parent_cap_fe)
          .field("parent_cap_frootid", &self.parent_cap_frootid)
          .field("parent_ouid", &self.parent_ouid)
          .field("parent_item", &self.parent_item)
          .field("parent_cap_fver", &self.parent_cap_fver)
          .field("parent_mode", &self.parent_mode)
          .field("parent_rdev", &self.parent_rdev)
          .field("parent_cap_fi", &self.parent_cap_fi)
          .field("parent_cap_fp", &self.parent_cap_fp)
          .field("parent_dev", &self.parent_dev)
          .field("parent_ogid", &self.parent_ogid)
          .field("cwd", &self.cwd)
          .field("syscall", &self.syscall)
          .field("ppid", &self.ppid)
          .field("comm", &self.comm)
          .field("fsuid", &self.fsuid)
          .field("pid", &self.pid)
          .field("a0", &self.a0)
          .field("a1", &self.a1)
          .field("a2", &self.a2)
          .field("a3", &self.a3)
          .field("arch", &self.arch)
          .field("auid", &self.auid)
          .field("items", &self.items)
          .field("gid", &self.gid)
          .field("euid", &self.euid)
          .field("sgid", &self.sgid)
          .field("uid", &self.uid)
          .field("tty", &self.tty)
          .field("success", &self.success)
          .field("exit", &self.exit)
          .field("ses", &self.ses)
          .field("key", &self.key)
          .field("suid", &self.suid)
          .field("egid", &self.egid)
          .field("fsgid", &self.fsgid)
          .field("exe", &self.exe)
          .finish()
    }
}

// ----------------------------------------------------------------------------

/*#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Event;
    use notify::op::Op;
    use std::path::PathBuf;
    use std::fs;

    // ------------------------------------------------------------------------

    fn remove_test_file(filename: String) {
        fs::remove_file(filename).unwrap()
    }

    fn create_test_event() -> Event {
        Event {
            id: "Test_id".to_string(),
            timestamp: "Timestamp".to_string(),
            hostname: "Hostname".to_string(),
            node: "FIM".to_string(),
            version: "x.x.x".to_string(),
            op: Op::CREATE,
            path: PathBuf::new(),
            labels: Vec::new(),
            operation: "TEST".to_string(),
            checksum: "UNKNOWN".to_string(),
            pid: 0,
            system: "test".to_string()
        }
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_create_event() {

    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_send_event() {

    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_operation(){

    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_event_fmt(){

    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_format_json() {

    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_log_event() {

    }
}*/