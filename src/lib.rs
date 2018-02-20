pub fn crypt(v: &mut Vec<u8>, key: &Vec<u8>) {
    for i in 0..v.len() {
        v[i] = v[i] ^ key[i % key.len()];
    }
}