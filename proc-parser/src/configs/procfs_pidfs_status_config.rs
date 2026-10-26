use super::config::ProcFSConfig;
use super::config::ProcFSConfigTrait;
use serde::Deserialize;
use std::collections::HashMap;

// its the status config
// its from the /proc/<PID>/status

#[derive(Debug, Default, Deserialize)]
pub struct ProcPIDStatusConfigFields {
    #[serde(rename = "Name")]
    pub name: bool,
    #[serde(rename = "Umask")]
    pub umask: bool,
    #[serde(rename = "State")]
    pub state: bool,
    #[serde(rename = "Tgid")]
    pub tgid: bool,
    #[serde(rename = "Ngid")]
    pub ngid: bool,
    #[serde(rename = "Pid")]
    pub pid: bool,
    #[serde(rename = "PPid")]
    pub ppid: bool,
    #[serde(rename = "TracerPid")]
    pub tracer_pid: bool,
    #[serde(rename = "Uid")]
    pub uid: bool,
    #[serde(rename = "Gid")]
    pub gid: bool,
    #[serde(rename = "FDSize")]
    pub fd_size: bool,
    #[serde(rename = "Groups")]
    pub groups: bool,
    #[serde(rename = "NStgid")]
    pub nstgid: bool,
    #[serde(rename = "NSpid")]
    pub nspid: bool,
    #[serde(rename = "NSpgid")]
    pub nspgid: bool,
    #[serde(rename = "NSsid")]
    pub nssid: bool,
    #[serde(rename = "Kthread")]
    pub kthread: bool,
    #[serde(rename = "VmPeak")]
    pub vm_peak: bool,
    #[serde(rename = "VmSize")]
    pub vm_size: bool,
    #[serde(rename = "VmLck")]
    pub vm_lck: bool,
    #[serde(rename = "VmPin")]
    pub vm_pin: bool,
    #[serde(rename = "VmHWM")]
    pub vm_hwm: bool,
    #[serde(rename = "VmRSS")]
    pub vm_rss: bool,
    #[serde(rename = "RssAnon")]
    pub rss_anon: bool,
    #[serde(rename = "RssFile")]
    pub rss_file: bool,
    #[serde(rename = "RssShmem")]
    pub rss_shmem: bool,
    #[serde(rename = "VmData")]
    pub vm_data: bool,
    #[serde(rename = "VmStk")]
    pub vm_stk: bool,
    #[serde(rename = "VmExe")]
    pub vm_exe: bool,
    #[serde(rename = "VmLib")]
    pub vm_lib: bool,
    #[serde(rename = "VmPTE")]
    pub vm_pte: bool,
    #[serde(rename = "VmSwap")]
    pub vm_swap: bool,
    #[serde(rename = "HugetlbPages")]
    pub hugetlb_pages: bool,
    #[serde(rename = "CoreDumping")]
    pub core_dumping: bool,
    #[serde(rename = "THP_enabled")]
    pub thp_enabled: bool,
    pub untag_mask: bool,
    #[serde(rename = "Threads")]
    pub threads: bool,
    #[serde(rename = "SigQ")]
    pub sigq: bool,
    #[serde(rename = "SigPnd")]
    pub sigpnd: bool,
    #[serde(rename = "ShdPnd")]
    pub shdpnd: bool,
    #[serde(rename = "SigBlk")]
    pub sigblk: bool,
    #[serde(rename = "SigIgn")]
    pub sigign: bool,
    #[serde(rename = "SigCgt")]
    pub sigcgt: bool,
    #[serde(rename = "CapInh")]
    pub capinh: bool,
    #[serde(rename = "CapPrm")]
    pub capprm: bool,
    #[serde(rename = "CapEff")]
    pub capeff: bool,
    #[serde(rename = "CapBnd")]
    pub capbnd: bool,
    #[serde(rename = "CapAmb")]
    pub capamb: bool,
    #[serde(rename = "NoNewPrivs")]
    pub nonewprivs: bool,
    #[serde(rename = "Seccomp")]
    pub seccomp: bool,
    #[serde(rename = "Seccomp_filters")]
    pub seccomp_filters: bool,
    #[serde(rename = "Speculation_Store_Bypass")]
    pub speculation_store_bypass: bool,
    #[serde(rename = "SpeculationIndirectBranch")]
    pub speculationindirectbranch: bool,
    #[serde(rename = "Cpus_allowed")]
    pub cpus_allowed: bool,
    #[serde(rename = "Cpus_allowed_list")]
    pub cpus_allowed_list: bool,
    #[serde(rename = "Mems_allowed")]
    pub mems_allowed: bool,
    #[serde(rename = "Mems_allowed_list")]
    pub mems_allowed_list: bool,
    pub voluntary_ctxt_switches: bool,
    pub nonvoluntary_ctxt_switches: bool,
    #[serde(rename = "x86_Thread_features")]
    pub x86_thread_features: bool,
    #[serde(rename = "x86_Thread_features_locked")]
    pub x86_thread_features_locked: bool,
}

pub type ProcPIDStatusConfig = ProcFSConfig<ProcPIDStatusConfigFields>;

impl ProcFSConfigTrait for ProcPIDStatusConfig {
    fn get_hashmap(&self) -> HashMap<usize, bool> {
        HashMap::from([
            (1, self.fields.name),
            (2, self.fields.umask),
            (3, self.fields.state),
            (4, self.fields.tgid),
            (5, self.fields.ngid),
            (6, self.fields.pid),
            (7, self.fields.ppid),
            (8, self.fields.tracer_pid),
            (9, self.fields.uid),
            (10, self.fields.gid),
            (11, self.fields.fd_size),
            (12, self.fields.groups),
            (13, self.fields.nstgid),
            (14, self.fields.nspid),
            (15, self.fields.nspgid),
            (16, self.fields.nssid),
            (17, self.fields.kthread),
            (18, self.fields.vm_peak),
            (19, self.fields.vm_size),
            (20, self.fields.vm_lck),
            (21, self.fields.vm_pin),
            (22, self.fields.vm_hwm),
            (23, self.fields.vm_rss),
            (24, self.fields.rss_anon),
            (25, self.fields.rss_file),
            (26, self.fields.rss_shmem),
            (27, self.fields.vm_data),
            (28, self.fields.vm_stk),
            (29, self.fields.vm_exe),
            (30, self.fields.vm_lib),
            (31, self.fields.vm_pte),
            (32, self.fields.vm_swap),
            (33, self.fields.hugetlb_pages),
            (34, self.fields.core_dumping),
            (35, self.fields.thp_enabled),
            (36, self.fields.untag_mask),
            (37, self.fields.threads),
            (38, self.fields.sigq),
            (39, self.fields.sigpnd),
            (40, self.fields.shdpnd),
            (41, self.fields.sigblk),
            (42, self.fields.sigign),
            (43, self.fields.sigcgt),
            (44, self.fields.capinh),
            (45, self.fields.capprm),
            (46, self.fields.capeff),
            (47, self.fields.capbnd),
            (48, self.fields.capamb),
            (49, self.fields.nonewprivs),
            (50, self.fields.seccomp),
            (51, self.fields.seccomp_filters),
            (52, self.fields.speculation_store_bypass),
            (53, self.fields.speculationindirectbranch),
            (54, self.fields.cpus_allowed),
            (55, self.fields.cpus_allowed_list),
            (56, self.fields.mems_allowed),
            (57, self.fields.mems_allowed_list),
            (58, self.fields.voluntary_ctxt_switches),
            (59, self.fields.nonvoluntary_ctxt_switches),
            (60, self.fields.x86_thread_features),
            (61, self.fields.x86_thread_features_locked),
        ])
    }

    fn get_field_string_to_struct_ids(&self) -> HashMap<&'static str, (usize, bool)> {
        HashMap::from([
            ("Name", (1, self.fields.name)),
            ("Umask", (2, self.fields.umask)),
            ("State", (3, self.fields.state)),
            ("Tgid", (4, self.fields.tgid)),
            ("Ngid", (5, self.fields.ngid)),
            ("Pid", (6, self.fields.pid)),
            ("PPid", (7, self.fields.ppid)),
            ("TracerPid", (8, self.fields.tracer_pid)),
            ("Uid", (9, self.fields.uid)),
            ("Gid", (10, self.fields.gid)),
            ("FDSize", (11, self.fields.fd_size)),
            ("Groups", (12, self.fields.groups)),
            ("NStgid", (13, self.fields.nstgid)),
            ("NSpid", (14, self.fields.nspid)),
            ("NSpgid", (15, self.fields.nspgid)),
            ("NSsid", (16, self.fields.nssid)),
            ("Kthread", (17, self.fields.kthread)),
            ("VmPeak", (18, self.fields.vm_peak)),
            ("VmSize", (19, self.fields.vm_size)),
            ("VmLck", (20, self.fields.vm_lck)),
            ("VmPin", (21, self.fields.vm_pin)),
            ("VmHWM", (22, self.fields.vm_hwm)),
            ("VmRSS", (23, self.fields.vm_rss)),
            ("RssAnon", (24, self.fields.rss_anon)),
            ("RssFile", (25, self.fields.rss_file)),
            ("RssShmem", (26, self.fields.rss_shmem)),
            ("VmData", (27, self.fields.vm_data)),
            ("VmStk", (28, self.fields.vm_stk)),
            ("VmExe", (29, self.fields.vm_exe)),
            ("VmLib", (30, self.fields.vm_lib)),
            ("VmPTE", (31, self.fields.vm_pte)),
            ("VmSwap", (32, self.fields.vm_swap)),
            ("HugetlbPages", (33, self.fields.hugetlb_pages)),
            ("CoreDumping", (34, self.fields.core_dumping)),
            ("THP_enabled", (35, self.fields.thp_enabled)),
            ("untag_mask", (36, self.fields.untag_mask)),
            ("Threads", (37, self.fields.threads)),
            ("SigQ", (38, self.fields.sigq)),
            ("SigPnd", (39, self.fields.sigpnd)),
            ("ShdPnd", (40, self.fields.shdpnd)),
            ("SigBlk", (41, self.fields.sigblk)),
            ("SigIgn", (42, self.fields.sigign)),
            ("SigCgt", (43, self.fields.sigcgt)),
            ("CapInh", (44, self.fields.capinh)),
            ("CapPrm", (45, self.fields.capprm)),
            ("CapEff", (46, self.fields.capeff)),
            ("CapBnd", (47, self.fields.capbnd)),
            ("CapAmb", (48, self.fields.capamb)),
            ("NoNewPrivs", (49, self.fields.nonewprivs)),
            ("Seccomp", (50, self.fields.seccomp)),
            ("Seccomp_filters", (51, self.fields.seccomp_filters)),
            (
                "Speculation_Store_Bypass",
                (52, self.fields.speculation_store_bypass),
            ),
            (
                "SpeculationIndirectBranch",
                (53, self.fields.speculationindirectbranch),
            ),
            ("Cpus_allowed", (54, self.fields.cpus_allowed)),
            ("Cpus_allowed_list", (55, self.fields.cpus_allowed_list)),
            ("Mems_allowed", (56, self.fields.mems_allowed)),
            ("Mems_allowed_list", (57, self.fields.mems_allowed_list)),
            (
                "voluntary_ctxt_switches",
                (58, self.fields.voluntary_ctxt_switches),
            ),
            (
                "nonvoluntary_ctxt_switches",
                (59, self.fields.nonvoluntary_ctxt_switches),
            ),
            ("x86_Thread_features", (60, self.fields.x86_thread_features)),
            (
                "x86_Thread_features_locked",
                (61, self.fields.x86_thread_features_locked),
            ),
        ])
    }

    fn is_enabled(&self) -> bool {
        self.allow
            & self.fields.name
            & self.fields.umask
            & self.fields.state
            & self.fields.tgid
            & self.fields.ngid
            & self.fields.pid
            & self.fields.ppid
            & self.fields.tracer_pid
            & self.fields.uid
            & self.fields.gid
            & self.fields.fd_size
            & self.fields.groups
            & self.fields.nstgid
            & self.fields.nspid
            & self.fields.nspgid
            & self.fields.nssid
            & self.fields.kthread
            & self.fields.vm_peak
            & self.fields.vm_size
            & self.fields.vm_lck
            & self.fields.vm_pin
            & self.fields.vm_hwm
            & self.fields.vm_rss
            & self.fields.rss_anon
            & self.fields.rss_file
            & self.fields.rss_shmem
            & self.fields.vm_data
            & self.fields.vm_stk
            & self.fields.vm_exe
            & self.fields.vm_lib
            & self.fields.vm_pte
            & self.fields.vm_swap
            & self.fields.hugetlb_pages
            & self.fields.core_dumping
            & self.fields.thp_enabled
            & self.fields.untag_mask
            & self.fields.threads
            & self.fields.sigq
            & self.fields.sigpnd
            & self.fields.shdpnd
            & self.fields.sigblk
            & self.fields.sigign
            & self.fields.sigcgt
            & self.fields.capinh
            & self.fields.capprm
            & self.fields.capeff
            & self.fields.capbnd
            & self.fields.capamb
            & self.fields.nonewprivs
            & self.fields.seccomp
            & self.fields.seccomp_filters
            & self.fields.speculation_store_bypass
            & self.fields.speculationindirectbranch
            & self.fields.cpus_allowed
            & self.fields.cpus_allowed_list
            & self.fields.mems_allowed
            & self.fields.mems_allowed_list
            & self.fields.voluntary_ctxt_switches
            & self.fields.nonvoluntary_ctxt_switches
            & self.fields.x86_thread_features
            & self.fields.x86_thread_features_locked
    }
}
