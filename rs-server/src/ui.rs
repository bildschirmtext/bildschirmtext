use crate::cept::Cept;

pub fn create_title(cept: &mut Cept, s: &str) {
    cept.add_ceptml("<csr:2,1><pal:1><sbgs:4><g0:g0><left:g0><mode:p><pal:0><9e><n><r><lbgs:4><n><lbgs:4><pal:1><height:2><r>");
    cept.add_str(s);
    cept.add_ceptml("<n><r><pal:0><size:1><9e><fgs:7>");
}
