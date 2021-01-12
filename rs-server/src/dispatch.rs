use std::collections::HashMap;
use crate::session::*;
use super::page::*;
use super::user::*;


pub trait PageSession<'a> {
    fn create(&self) -> Option<Page>;
    fn validate(&self, name: &str, input_data: &HashMap<String, String>) -> ValidateResult;
    fn send(&self, input_data: &HashMap<String, String>) -> UserRequest;
}

struct PageSessionNewFn(fn(&str, PageId, User) -> Box<dyn PageSession<'static>>);

// mask:
//   * If a mask does not end in '*' or '-', the page number must match exactly.
//   * If a mask ends in '*', it only has to be a prefix of the page number.
//   * If a mask ends in '-', it only has to be a prefix of the page number. The prefix of the
//     page number will be stripped when passed into the function.
// bool:
//   * Only use 'true' for BTX-internal pages that need to access the
//     user's info and statistics!
// N.B.: The table must be in the right order: longer prefixes must come first!
const DISPATCH_TABLE: &[(&[u8], bool, PageSessionNewFn, &str, &str)] = &[
    (b"00000*", true,  PageSessionNewFn(super::login::new),        "", ""),
    (b"9",      true,  PageSessionNewFn(super::login::new),        "", ""),
    (b"8*",     true,  PageSessionNewFn(super::ui_messaging::new), "", ""),
    (b"77",     false, PageSessionNewFn(super::ui_user::new),      "", ""),
    (b"7-",     false, PageSessionNewFn(super::historic::new),     "", ""),
    (b"666",    false, PageSessionNewFn(super::image::new),        "", ""),

    // static pages
    (b"0-",     false, PageSessionNewFn(super::staticp::new),      "../data/0/", ""),

    // historic pages #1
    (b"1050-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/1050/", "Btx-Telex"),
    (b"1188-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/1188/", "Postreklame"),
    (b"1690-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/1690/", "Bildschirmtext"),
    (b"1692-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/1692/", "Btx-Cityruf"),
    (b"20000-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/20000/", "Deutsche Bundespost"),
    (b"20095-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/20095/", "Commodore Büromaschinen GmbH"),
    (b"20096-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/20096/", "Btx-Demo AMIGA"),
    (b"20511-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/20511/", "Verlag M. DuMont Schauberg"),
    (b"21212-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/21212/", "Verbraucher-Zentrale NRW e. V."),
    (b"25800-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/25800/", "Deutsche Bundesbahn"),
    (b"30003-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/30003/", "Formel Eins"),
    (b"30711-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/30711/", "Btx Südwest Datenbank GmbH"),
    (b"33033-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/33033/", "Eden"),
    (b"34034-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/34034/",  "Frankfurter Allgemeine Zeitung"),
    (b"34344-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/34344/", "Neue Mediengesellschaft Ulm"),
    (b"35853-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/35853/", "ABIDA GmbH"),
    (b"40040-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/40040/", "Axel Springer Verlag"),
    (b"44479-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/44479/", "DIMDI"),
    (b"50257-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/50257/", "Computerwelt Btx-Info-Dienst"),
    (b"54004-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/54004/", "ÖVA Versicherungen"),
    (b"57575-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/57575/", "Lotto Toto"),
    (b"64064-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/64064/", "Markt & Technik"),
    (b"65432-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/65432/", "ADAC"),
    (b"67007-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/67007/", "Rheinpfalz Verlag u. Druckerei"),
    (b"201474-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/201474/", "Rhein-Neckar-Zeitung"),
    (b"208585-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/208585/", "eba Pressebüro und Verlag"),
    (b"208888-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/208888/", "Neue Mediengesellschaft Ulm"),
    (b"402060-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/402060/", "AUTO & BTX WOLFSBURG"),
    (b"8211882-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/8211882/", "Postreklame"),
    (b"12001551-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/12001551/", "Neue Mediengesellschaft Ulm"),
    (b"50707545-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/50707545/", "Vogel-Verlag KG"),
    (b"86553222-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/86553222/", "Chaos Computer Club"),
    (b"505050035-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/505050035/", "Institut für Btx und Telematik"),
    (b"2000014317-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/2000014317/", "DBP PGiroA Essen"),
    (b"15148830101-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/15148830101/", "DBP Fernmeldeamt 2 Düsseldorf"),
    (b"920492040092-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/920492040092/", "Wolfgang Fritsch"),
    (b"1180040000004-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/1180040000004/", "Elektronisches Telefonbuch"),
    (b"1200833401083-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/1200833401083/", "Deutsche Bundesbahn"),
    // historic pages #2
    (b"00000-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/11/00000/", "Telekom Datex-J"),
    (b"20111-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/11/20111/", "VOBIS MICROCOMPUTER AG"),
    (b"21199-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/11/21199/", "MICROSOFT GMBH"),
    (b"25800-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/11/25800/", "Deutsche Bundesbahn"),
    (b"28000-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/11/28000/", "Postbank"),
    (b"34561-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/11/34561/", "1&1 TELEKOMMUNIKATION GMBH"),
    (b"37107-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/11/37107/", "WDR Computer-Club"),
    (b"46801-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/11/46801/", "Verlagsgruppe Handelsblatt"),
    (b"49498-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/11/49498/", "Stellfeldt-Forche Btx-Agentur"),
    (b"50000-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/11/50000/", "Deutsche Lufthansa AG"),
    (b"52800-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/11/52800/", "IBM Deutschland"),
    (b"58587-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/11/58587/", "ITZ GmbH"),
    (b"69010-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/11/69010/", "Deutscher Ind. Handelstag DIHT"),
    (b"353535-",  false, PageSessionNewFn(super::staticp::new),     "../data/hist/11/353535/", "START Telematik Tourismus Info"),

    (b"*",  false, PageSessionNewFn(super::staticp::new),      "", ""), // will return None
];

pub fn dispatch_pageid<'a>(pageid: &PageId, user: &User, anonymous_user: &User) -> (&'static str, Box<dyn PageSession<'static>>) {
    for (mask, private_data, new_fn, arg, publisher_name) in DISPATCH_TABLE {
        let matches;
        let reduce;
        let last = *mask.last().unwrap();
        if last == b'*' || last == b'-' {
            let mask = std::str::from_utf8(&mask[0..mask.len() - 1]).unwrap();
            matches = pageid.page.starts_with(mask);
            reduce = match last {
                b'*' => 0,
                _    => mask.len(),
            };
        } else {
            matches = pageid.page == std::str::from_utf8(mask).unwrap();
            reduce = 0;
        };
        if matches {
            let pageid = pageid.reduced_by(reduce).clone();
            let user = if *private_data { user } else { anonymous_user };
            return (publisher_name, new_fn.0(arg, pageid, user.clone()));
        }
    }
    unreachable!();
}
