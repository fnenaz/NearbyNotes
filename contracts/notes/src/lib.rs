#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struct data
#[contracttype]
#[derive(Clone, Debug)]
pub struct GirlNote {
    nama: String,
    idline: u64,
    jarak: u32,
    rating: u32, // skala 1 - 10
}

// Storage key
const NOTE_DATA: Symbol = symbol_short!("NOTE_DATA");

#[contract]
pub struct NotesContract;

#[contractimpl]
impl NotesContract {

    // READ (ambil semua data)
    pub fn read(env: Env) -> Vec<GirlNote> {
        env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env))
    }

    // CREATE (tambah data baru)
    pub fn create(env: Env, nama: String, idline: u64, jarak: u32, rating: u32) -> String {
        let mut data: Vec<GirlNote> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

        let note = GirlNote {
            nama,
            idline,
            jarak,
            rating,
        };

        data.push_back(note);
        env.storage().instance().set(&NOTE_DATA, &data);

        String::from_str(&env, "Data berhasil ditambahkan")
    }

    // DELETE (hapus berdasarkan idline)
    pub fn delete(env: Env, idline: u64) -> String {
        let mut data: Vec<GirlNote> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

        for i in 0..data.len() {
            if data.get(i).unwrap().idline == idline {
                data.remove(i);
                env.storage().instance().set(&NOTE_DATA, &data);
                return String::from_str(&env, "Data berhasil dihapus");
            }
        }

        String::from_str(&env, "Data tidak ditemukan")
    }
}
mod test;