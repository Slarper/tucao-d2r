use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};
use json_comments::StripComments;

use regex::Regex;


fn main() {
    // merge_overlap(
    //     "assets/item-names - tucao.json",
    //     "assets/item-names - tucao - tina.json",
    //     "assets/item-names - tucao - merge.json")

    infuse_tucao(
        "assets/tucao/item-names - tucao - merge.json", 
        "assets/Dusk/item-names - basic filter.json", 
        "work/item-names -basic -tucao.json");

    // extract_ignore(
    //     "assets/Dusk/item-names - standard filter.json", 
    //     "assets/ignore/item-names -i.json"
    // )

    infuse_ignore(
        "assets/ignore/item-names -i.json",
        "work/item-names -basic -tucao.json",
        "work/item-names -std -tucao.json"
    );
}

// fn main(){
//     b2a("work/names1.json", "work/names-tucao.json", "work/names-o.json");
//     b2a("work/runes1.json", "work/runes-tucao.json", "work/runes-o.json");
// }

// fn main(){

//     let src = read("work/item-names - tina.json");

//     let mut tucaos = vec![];

//     for item in src {

//         if let Some(zh_cn) = &item.zhTW {
//             match get_tu_cao(&zh_cn, 0) {
//                 Some(tu_cao) => {
//                     let tucao_item = ItemOption {
//                         id: item.id,
//                         Key: item.Key.clone(),
//                         zhCN: Some(tu_cao.clone()),
//                         .. Default::default()
//                     };

//                     tucaos.push(tucao_item)
    
//                 },
//                 None => {}
//             }
//         }

//     }

//     save("assets/item-names - tucao - tina.json", tucaos)
//     // extract_tucao("work/item-names - tina.json", "assets/item-names - tucao - tina.json");
// }


fn infuse_tucao(tucao:&str, names:&str, o:&str) {
    let tucao = read(tucao);
    let names = read(names);

    let mut names: HashMap<i32, ItemOption> = vec_to_map(names);
    for item in tucao {
        let tu_cao = item.zhCN.unwrap();
        let item2 = names.get_mut(&item.id).unwrap();

        let zh_cn = item2.zhCN.clone().unwrap();
        let mut zh_cn = parse_name(zh_cn);
        zh_cn.insert(1, tu_cao.clone());
        let zh_cn = encode_name(zh_cn);
        item2.zhCN = Some(zh_cn);

        let zh_cn = item2.enUS.clone().unwrap();
        let mut zh_cn = parse_name(zh_cn);
        zh_cn.insert(1, tu_cao.clone());
        let zh_cn = encode_name(zh_cn);
        item2.enUS = Some(zh_cn);
    }

    let b = map_to_vec(names);
    
    save(o, b)
}

/// names-in-d2 are reversely displayed by '\n'
fn parse_name(n: String) -> Vec<String>{
    let mut v = n.split("\n").map(|s| s.to_string()).collect::<Vec<String>>();
    v.reverse();
    v
}

fn encode_name(mut v: Vec<String>) -> String {
    v.reverse();
    v.join("\n")
}

fn  infuse_ignore(ig:&str, names:&str, o:&str) {
    let ig = read(ig);
    let names = read(names);

    let mut names: HashMap<i32, ItemOption> = vec_to_map(names);
    for item in ig {
        let item2 = names.get_mut(&item.id).unwrap();
        item2.zhCN = Some(String::new());
        item2.enUS = Some(String::new());
        item2.zhTW = Some(String::new());

    }

    let b = map_to_vec(names);
    
    save(o, b)
}

/// merge a to (over) b then output to o
fn merge_overlap(a:&str, b:&str, o:&str) {
    let a = read(a);
    let b = read(b);

    let mut b = vec_to_map(b);

    for item in a {
        b.insert(item.id, item.clone());
    }

    let b = map_to_vec(b);
    
    save(o, b)
}

fn vec_to_map(a:Vec<ItemOption>) -> HashMap<i32, ItemOption> {
    let mut a = {
        let mut h = HashMap::new();
        for item in a{
            h.insert(item.id, item);
        }
        h
    };
    a
}

fn map_to_vec(h:HashMap<i32, ItemOption>) -> Vec<ItemOption> {
    let a_r = {
        let mut v = vec![];
        for item in h.values() {
            v.push(item.clone())
        }
        v
    };

    a_r
}

fn b2a(a:&str, b:&str, c:&str) {

    // must be pure utf-8 encoding
    let a = fs::read_to_string(a).unwrap();
    // let a = fs::read_to_string("work/item-names - no filter t.json").unwrap();


    let b = fs::read_to_string(b).unwrap();
    let a = StripComments::new(a.as_bytes());

    let a:Vec<Item> = serde_json::from_reader(a).unwrap();
    // dbg!(&a);
    let b:Vec<Item> = serde_json::from_str(&b).unwrap();

    let mut a = {
        let mut h = HashMap::new();
        for item in a{
            h.insert(item.id, item);
        }
        h
    };

    for item in b {
        match get_tu_cao(&item.zhCN, 1) {
            Some(tu_cao) => {
                let a_item = a.get_mut(&item.id).unwrap();
                a_item.zhCN = vec![tu_cao.clone(), a_item.zhCN.clone()].join("\n");
                a_item.enUS = vec![tu_cao.clone(), a_item.enUS.clone()].join("\n");

            },
            None => {}
        }
    }

    let a_r = {
        let mut v = vec![];
        for item in a.values() {
            v.push(item.clone())
        }
        v
    };

    let a_s = serde_json::to_string_pretty(&a_r).unwrap();

    fs::write(c, a_s).unwrap();


}


#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    data: String,
}

// pos : 1
fn get_tu_cao (name: &str, pos:usize) -> Option<String> {
    let n = name.split("\n").collect::<Vec<&str>>();
    if n.len() == 3 {

        // strip blizzard string color code (ÿc_)
        let tu_cao = n[pos];
        let re = Regex::new(r"ÿc.").unwrap();
        let tu_cao = re.replace_all(tu_cao, "");
        Some(tu_cao.to_string())
        
    } else {
        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    id: i32,
    Key: String,
    enUS: String,
    zhTW: String,
    deDE: String,
    esES: String,
    frFR: String,
    itIT: String,
    koKR: String,
    plPL: String,
    esMX: String,
    jaJP: String,
    ptBR: String,
    ruRU: String,
    zhCN: String,
    
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ItemOption {
    id: i32,
    Key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    enUS: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zhTW: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deDE: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    esES: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frFR: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    itIT: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    koKR: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plPL: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    esMX: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jaJP: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ptBR: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ruRU: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zhCN: Option<String>,
    
    
}

/// it will strip comment
// warning: utf-8 only !!!!!
// utf-8 with BOM is not OK :)
fn read(src:&str) -> Vec<ItemOption> {
    let src: String = fs::read_to_string(src).unwrap();
    let src = StripComments::new(src.as_bytes());
    let src:Vec<ItemOption> = serde_json::from_reader(src).unwrap();
    src
}

fn save(tgt:&str, tucaos: Vec<ItemOption>) {
    let tucaos = serde_json::to_string_pretty(&tucaos).unwrap();

    fs::write(tgt, tucaos).unwrap();
}

impl Default for ItemOption {
    fn default() -> Self {
        ItemOption {
            id: 0,
            Key: String::new(),
            zhCN: None,
            zhTW: None,
            enUS: None,
            itIT: None,
            deDE: None,
            frFR: None,
            jaJP: None,
            esMX: None,
            plPL: None,
            ruRU:None,
            esES:None,
            koKR:None,
            ptBR:None
        }
    }
}

fn extract_tucao(src:&str, tgt:&str) {



    let src: String = fs::read_to_string(src).unwrap();

    let src = StripComments::new(src.as_bytes());


    let src:Vec<ItemOption> = serde_json::from_reader(src).unwrap();

    let mut tucaos = Vec::new();

    for item in src {

        if let Some(zh_cn) = &item.zhCN {
            match get_tu_cao(&zh_cn, 1) {
                Some(tu_cao) => {
                    let tucao_item = ItemOption {
                        id: item.id,
                        Key: item.Key.clone(),
                        zhCN: Some(tu_cao.clone()),
                        .. Default::default()
                    };

                    tucaos.push(tucao_item)
    
                },
                None => {}
            }
        }

    }



    let tucaos = serde_json::to_string_pretty(&tucaos).unwrap();

    fs::write(tgt, tucaos).unwrap();

}

fn extract_ignore(src:&str, tgt:&str) {

    let src = read(src);

    let mut o = Vec::new();

    for item in src {
        if is_empty(item.zhCN) && is_empty(item.zhTW) && is_empty(item.enUS) {
            let item2 = ItemOption{
                id: item.id,
                Key: item.Key,
                ..Default::default()
            };

            o.push(item2)
        }
    }

    save(tgt, o)

}

fn is_empty(s: Option<String>) -> bool {
    match s {
        None => true,
        Some(s) => {
            s.is_empty()
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct ItemList{
    data:Vec<Item>
}

impl ItemList {
    fn from_vec(vec: Vec<Item>) -> Self{
        ItemList { data: vec }
    }
}
