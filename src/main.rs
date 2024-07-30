mod clients;
mod file_processing;
mod bencode_processing;
mod utils;
mod torrent_manager;

use file_processing::filereader;
use torrent_manager::torrent_manager::TorrentManager;
use std::env;

use bencode_processing::decoder::decode_bencoded_value;
use bencode_processing::encoder::encode_bencoded_value;



// Main function to handle command-line arguments and execute commands
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let mut torrent_manager = TorrentManager::new(&encode_bencoded_value, &decode_bencoded_value);

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value.as_bytes(), true);
        println!("{}", decoded_value.unwrap().0.to_string());
        
    } else if command == "info" {
        let file = &args[2];
        let content = filereader::read_file_as_vector(file).unwrap();
        let _ = torrent_manager.parse_meta_info_file(content);
        let _ = torrent_manager.print_meta_info();

    } else if command == "peers" {
        let file = &args[2];
        let content = filereader::read_file_as_vector(file).unwrap();
        let _ = torrent_manager.parse_meta_info_file(content);
        let _ = torrent_manager.init_clients();
        let _ = torrent_manager.print_peers();

    } else if command == "handshake" {
        let file = &args[2];
        let content = filereader::read_file_as_vector(file).unwrap();
        let _ = torrent_manager.parse_meta_info_file(content);
        let _ = torrent_manager.init_clients();
        let resp = torrent_manager.perform_peer_handshake(&args[3]).await.unwrap();
        print!("Peer ID: {}\n",hex::encode(resp[48..].to_vec()));
    } else if command == "download_piece" {
        // read -o flag: path where to save the piece
        // two positional arguments
        // - torrentfile name
        // - piece number
        // e.g. ./your_bittorrent.sh download_piece -o /tmp/test-piece-0 sample.torrent 0
        // TODO: #1 Read all arguments as variables

        let output_path = &args[3];


        let file = &args[4];
        let content = filereader::read_file_as_vector(file).unwrap();
        let _ = torrent_manager.parse_meta_info_file(content);
        let _ = torrent_manager.init_clients();
        let piece = torrent_manager.download_piece_with_index(args[5].parse::<u32>().unwrap()).await;
        filereader::write_vector_to_file(&args[3], piece.unwrap());
        print!("Piece 0 downloaded to {}", &args[3])

    } else {
        println!("unknown command: {}", args[1])
    }
}



