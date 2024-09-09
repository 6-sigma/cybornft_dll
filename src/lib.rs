use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde_hex::{SerHex, StrictPfx};

use scale_info::TypeInfo;

use hex;
use std::io::{self, Write};

pub type TokenId = u128;

#[derive(Debug, Encode, Decode, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, TypeInfo)]
pub struct ActorId(
    #[serde(with = "SerHex::<StrictPfx>")]
    [u8; 32]
);

#[derive(PartialEq,Debug, Encode, Decode, Serialize, Deserialize, TypeInfo)]
pub struct Config {
    pub max_mint_count: Option<u128>,
    pub game_actor: ActorId,
}

#[derive(PartialEq,Debug, Encode, Decode, Serialize, Deserialize, TypeInfo)]
pub struct Collection {
    pub name: String,
    pub description: String,
}

#[derive(PartialEq,Debug, Encode, Decode,  Serialize, Deserialize, Clone, TypeInfo)]
pub enum CyborRace {
    MalikAhmed (u8),
    IsabellaRodriguez (u8),
}

impl Default for CyborRace {
    fn default() -> Self {
        CyborRace::MalikAhmed(0)
    }
}

#[derive(PartialEq,Default, Debug, Encode, Decode, Serialize, Deserialize, Clone, TypeInfo)]
pub struct TokenMetadata {
    // ex. "CryptoKitty #100"
    pub name: String,
    // free-form description
    pub description: String,
    // URL to associated media, preferably to decentralized, content-addressed storage
    pub media: String,
    // URL to an off-chain JSON file with more info.
    pub reference: String,
    
    // Game attributes
    pub race: CyborRace,

    // init_xxx * (level ** grade) / grade / K
    pub init_attack: u32,
    pub init_defence: u32,
    pub init_intelligence: u32,
    pub init_miners_limit: u16,

    pub level_limit: u8,
    pub grade_limit: u8,

}

#[derive(PartialEq, Debug, Encode, Decode, Serialize, Deserialize, TypeInfo)]
pub struct State {
    pub owner_by_id: Vec<(TokenId, ActorId)>,
    pub token_approvals: Vec<(TokenId, ActorId)>,
    pub token_metadata_by_id: Vec<(TokenId, TokenMetadata)>,
    pub tokens_for_owner: Vec<(ActorId, Vec<TokenId>)>,
    pub is_gaming: Vec<(TokenId, bool)>,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub collection: Collection,
    pub config: Config,
    pub level: u8,
    pub grade: u8,
}

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn hex_to_state_json(hex_str: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!hex_str.is_null());
        CStr::from_ptr(hex_str)
    };
    let hex_str = c_str.to_str().unwrap();
    let bytes = hex::decode(hex_str).expect("Decoding failed");
    let state: State = Decode::decode(&mut &bytes[..]).expect("Decoding State failed");
    let json_str = serde_json::to_string(&state).expect("Serialization failed");
    let c_string = CString::new(json_str).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn hex_to_tokens_by_owner_json(hex_str: *const c_char) -> *mut c_char {
        let c_str = unsafe {
        assert!(!hex_str.is_null());
        CStr::from_ptr(hex_str)
    };
    let hex_str = c_str.to_str().unwrap();
    let bytes = hex::decode(hex_str).expect("Decoding failed");
    let tokens: Option<Vec<TokenId>> = Decode::decode(&mut &bytes[..]).expect("Decoding State failed");
    let json_str = serde_json::to_string(&tokens).expect("Serialization failed");
    let c_string = CString::new(json_str).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn free_c_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe {
        CString::from_raw(s);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    // use std::os::raw::c_char;

    #[test]
    fn test_hex_to_state_json() {
        // 创建一个示例 State 结构体
        let state: State = State {
            owner_by_id: vec![],
            token_approvals: vec![],
            token_metadata_by_id: vec![],
            tokens_for_owner: vec![],
            is_gaming: vec![],
            token_id: 1,
            owner: ActorId([0; 32]),
            collection: Collection {
                name: "Test Collection".to_string(),
                description: "This is a test collection".to_string(),
            },
            config: Config {
                max_mint_count: Some(100),
                game_actor: ActorId([0; 32]),
            },
            level: 1,
            grade: 1,
        };


        let mut encoded_state = vec![];
        state.encode_to(&mut encoded_state);

        // let hex_str: String = hex::encode(&encoded_state);
        let hex_str = "140400000000000000000000000000000004d467c76fd92d080a31f88829652e6dc5586060fc66fb27d02ab54cee6943610000000000000000000000000000000004d467c76fd92d080a31f88829652e6dc5586060fc66fb27d02ab54cee69436102000000000000000000000000000000109ab78f2f10993f5e1354fff7ee99a3fa635dd5fe5968515a0b84471c43e89201000000000000000000000000000000109ab78f2f10993f5e1354fff7ee99a3fa635dd5fe5968515a0b84471c43e8920300000000000000000000000000000004d467c76fd92d080a31f88829652e6dc5586060fc66fb27d02ab54cee6943610c04000000000000000000000000000000fe6b97b0b5f9a74233cbf39ecb6b79ea604f66915c1a1e5d5cfdc32f3178366703000000000000000000000000000000fe6b97b0b5f9a74233cbf39ecb6b79ea604f66915c1a1e5d5cfdc32f3178366700000000000000000000000000000000fe6b97b0b5f9a74233cbf39ecb6b79ea604f66915c1a1e5d5cfdc32f317836671404000000000000000000000000000000244379626f722d3333320450c10168747470733a2f2f617a7572652d66726167696c652d636c6f776e666973682d3430342e6d7970696e6174612e636c6f75642f697066732f516d663366633569533378676877436e663243524177427150577378634b74393833597774554d446a6f3279386b2f31313132332e706e67c10168747470733a2f2f617a7572652d66726167696c652d636c6f776e666973682d3430342e6d7970696e6174612e636c6f75642f697066732f516d663366633569533378676877436e663243524177427150577378634b74393833597774554d446a6f3279386b2f31313132332e706e6700017b000000410100000c0000001600161600000000000000000000000000000000244359424f522d30303108532bc10168747470733a2f2f617a7572652d66726167696c652d636c6f776e666973682d3430342e6d7970696e6174612e636c6f75642f697066732f516d663366633569533378676877436e663243524177427150577378634b74393833597774554d446a6f3279386b2f31323233312e706e67c10168747470733a2f2f617a7572652d66726167696c652d636c6f776e666973682d3430342e6d7970696e6174612e636c6f75642f697066732f516d663366633569533378676877436e663243524177427150577378634b74393833597774554d446a6f3279386b2f31323233312e706e67000a6400000014000000640000000a00640a02000000000000000000000000000000204379626f722d39390450c10168747470733a2f2f617a7572652d66726167696c652d636c6f776e666973682d3430342e6d7970696e6174612e636c6f75642f697066732f516d663366633569533378676877436e663243524177427150577378634b74393833597774554d446a6f3279386b2f33323231332e706e67c10168747470733a2f2f617a7572652d66726167696c652d636c6f776e666973682d3430342e6d7970696e6174612e636c6f75642f697066732f516d663366633569533378676877436e663243524177427150577378634b74393833597774554d446a6f3279386b2f33323231332e706e6700017b000000410100000c0000001600161601000000000000000000000000000000244359424f522d303033082b39c10168747470733a2f2f617a7572652d66726167696c652d636c6f776e666973682d3430342e6d7970696e6174612e636c6f75642f697066732f516d663366633569533378676877436e663243524177427150577378634b74393833597774554d446a6f3279386b2f33323231332e706e67c10168747470733a2f2f617a7572652d66726167696c652d636c6f776e666973682d3430342e6d7970696e6174612e636c6f75642f697066732f516d663366633569533378676877436e663243524177427150577378634b74393833597774554d446a6f3279386b2f33323231332e706e670003e8030000c8000000e8030000c800641403000000000000000000000000000000204379626f722d39390450c10168747470733a2f2f617a7572652d66726167696c652d636c6f776e666973682d3430342e6d7970696e6174612e636c6f75642f697066732f516d663366633569533378676877436e663243524177427150577378634b74393833597774554d446a6f3279386b2f31313232332e706e67c10168747470733a2f2f617a7572652d66726167696c652d636c6f776e666973682d3430342e6d7970696e6174612e636c6f75642f697066732f516d663366633569533378676877436e663243524177427150577378634b74393833597774554d446a6f3279386b2f31313232332e706e6700017b000000410100000c0000001600161608109ab78f2f10993f5e1354fff7ee99a3fa635dd5fe5968515a0b84471c43e89208020000000000000000000000000000000100000000000000000000000000000004d467c76fd92d080a31f88829652e6dc5586060fc66fb27d02ab54cee6943610c0400000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000010500000000000000000000000000000004d467c76fd92d080a31f88829652e6dc5586060fc66fb27d02ab54cee69436144365349474d4156455253452d4359424f52244359424f522d4d414e016400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        println!("hex_str: {}", hex_str); // 打印 hex_str
        std::io::stdout().flush().unwrap(); // 刷新标准输出

        let c_hex_str = CString::new(hex_str).unwrap();
        let json_c_str = hex_to_state_json(c_hex_str.as_ptr());
        // let json_c_str = hex_to_tokens_by_owner_json(c_hex_str.as_ptr());
        println!("json_c_str: {:?}", json_c_str);
        std::io::stdout().flush().unwrap();
        let json_str = unsafe { CString::from_raw(json_c_str).into_string().unwrap() };
        println!("json_str: {}", json_str); // 打印 json_str
        std::io::stdout().flush().unwrap();

    }

    #[test]
    fn test_free_c_string() {
        let c_str = CString::new("Test String").unwrap();
        let ptr = c_str.into_raw();
        free_c_string(ptr);
    }
}