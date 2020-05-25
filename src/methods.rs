use serde::{Serialize, de::DeserializeOwned};

use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, SocketAddr};

use deluge_rpc_macro::rpc_class;

use crate::types::{Result, TorrentOptions, AuthLevel, InfoHash, Query, EventKind};
use crate::session::Session;

rpc_class! {
    impl Session::daemon;

    #[rpc(method = "info", auth_level = "Nobody")]
    pub rpc fn daemon_info(&mut self) -> String;

    #[rpc(auth_level = "Nobody", client_version = "2.0.4.dev23")]
    pub rpc fn login(&mut self, username: &str, password: &str) -> AuthLevel {
        self.auth_level = val;
        Ok(self.auth_level)
    }

    #[rpc(method = "set_event_interest")]
    rpc fn _set_event_interest(&mut self, events: &[EventKind]) -> bool;

    pub async fn set_event_interest(&mut self, events: &HashSet<EventKind>) -> Result<bool> {
        // TODO: Error variant for incorrect crate usage, like here.
        assert!(self.event_receiver_count() > 0, "Cannot set event interest without an active receiver handle (try calling .subscribe_events() first)");
        let keys: Vec<EventKind> = events.iter().copied().collect();
        self._set_event_interest(&keys).await
    }

    pub rpc fn shutdown(&mut self) -> ();

    pub rpc fn get_method_list(&mut self) -> Vec<String>;

    pub rpc fn get_version(&mut self) -> String;

    #[rpc(auth_level = "ReadOnly")]
    pub rpc fn authorized_call(&mut self, rpc: &str) -> bool;
}

rpc_class! {
    impl Session::core;

    pub rpc fn add_torrent_file(&mut self, filename: &str, filedump: &str, options: &TorrentOptions) -> Option<InfoHash>;

    pub rpc fn add_torrent_files(&mut self, torrent_files: &[(&str, &str, &TorrentOptions)]);

    // TODO: accept a guaranteed-valid struct for the magnet URI
    pub rpc fn add_torrent_magnet(&mut self, uri: &str, options: &TorrentOptions) -> InfoHash;

    // TODO: accept guaranteed-valid structs for the URL and HTTP headers
    pub rpc fn add_torrent_url(&mut self, url: &str, options: &TorrentOptions, headers: Option<HashMap<String, String>>) -> Option<InfoHash>;

    #[rpc(method = "connect_peer")]
    rpc fn _connect_peer(&mut self, torrent_id: InfoHash, peer_ip: IpAddr, port: u16);

    pub async fn connect_peer(&mut self, torrent_id: InfoHash, peer_addr: SocketAddr) -> Result<()> {
        self._connect_peer(torrent_id, peer_addr.ip(), peer_addr.port()).await
    }

    #[rpc(auth_level = "Admin")]
    pub rpc fn create_account(&mut self, username: &str, password: &str, auth_level: AuthLevel);

    // FORGIVE ME: I have no idea whether these types are correct.
    pub rpc fn create_torrent(
        &mut self,
        path: &str,
        comment: &str,
        target: &str,
        webseeds: &[&str],
        private: bool,
        created_by: &str,
        add_to_session: bool,
    );

    pub rpc fn disable_plugin(&mut self, plugin: &str);

    pub rpc fn enable_plugin(&mut self, plugin: &str);

    pub rpc fn force_reannounce(&mut self, torrent_ids: &[InfoHash]);

    pub rpc fn force_recheck(&mut self, torrent_ids: &[InfoHash]);

    // I hardcoded these same mappings into the AuthLevel enum, so this isn't particularly useful, but hey
    pub rpc fn get_auth_levels_mappings(&mut self) -> (HashMap<String, AuthLevel>, HashMap<AuthLevel, String>);

    pub rpc fn get_config<T: DeserializeOwned>(&mut self) -> HashMap<String, T>;

    pub rpc fn get_config_value<T: DeserializeOwned>(&mut self, key: &str) -> T;

    // TODO: ConfigQuery trait and/or ConfigKey enum
    pub rpc fn get_config_values<T: DeserializeOwned>(&mut self, keys: &[&str]) -> HashMap<String, T>;

    pub rpc fn get_enabled_plugins(&mut self) -> Vec<String>;

    pub rpc fn get_external_ip(&mut self) -> IpAddr;

    pub rpc fn get_filter_tree(&mut self, show_zero_hits: bool, hide_cat: &[&str]) -> HashMap<String, Vec<(String, u64)>>;

    pub rpc fn get_free_space(&mut self, path: Option<&str>) -> u64;

    // TODO: Account struct
    #[rpc(auth_level = "Admin")]
    pub rpc fn get_known_accounts<T: DeserializeOwned>(&mut self) -> Vec<HashMap<String, T>>;

    pub rpc fn get_libtorrent_version(&mut self) -> String;

    pub rpc fn get_listen_port(&mut self) -> u16;

    // Uses a return value of -1 rather than a proper error to indicate filesystem errors.
    // Why? Is this a thin wrapper for some system call?
    pub rpc fn get_path_size(&mut self, path: &str) -> i64;

    // TODO: Proxy struct (low importance)
    pub rpc fn get_proxy<T: DeserializeOwned>(&mut self) -> T;

    pub rpc fn get_session_state(&mut self) -> Vec<InfoHash>;

    // TODO: SessionQuery trait and/or SessionKey enum
    pub rpc fn get_session_status<T: DeserializeOwned>(&mut self, keys: &[&str]) -> HashMap<String, T>;

    #[rpc(method = "get_torrent_status")]
    pub rpc fn get_torrent_status_dyn<T: DeserializeOwned>(&mut self, torrent_id: InfoHash, keys: &[&str], diff: bool) -> T;

    pub async fn get_torrent_status<T: Query>(&mut self, torrent_id: InfoHash) -> Result<T> {
        self.get_torrent_status_dyn(torrent_id, T::keys(), false).await
    }

    pub async fn get_torrent_status_diff<T: Query>(&mut self, torrent_id: InfoHash) -> Result<T::Diff> {
        self.get_torrent_status_dyn(torrent_id, T::keys(), true).await
    }

    #[rpc(method = "get_torrents_status")]
    pub rpc fn get_torrents_status_dyn<T: DeserializeOwned, U: Serialize>(&mut self, filter_dict: Option<HashMap<String, U>>, keys: &[&str], diff: bool) -> HashMap<InfoHash, T>;

    pub async fn get_torrents_status<T: Query, U: Serialize>(&mut self, filter_dict: Option<HashMap<String, U>>) -> Result<HashMap<InfoHash, T>> {
        self.get_torrents_status_dyn(filter_dict, T::keys(), false).await
    }

    pub async fn get_torrents_status_diff<T: Query, U: Serialize>(&mut self, filter_dict: Option<HashMap<String, U>>) -> Result<HashMap<InfoHash, T::Diff>> {
        self.get_torrents_status_dyn(filter_dict, T::keys(), true).await
    }

    pub rpc fn glob(&mut self, path: &str) -> Vec<String>;

    #[rpc(method = "is_session_paused")]
    pub rpc fn is_libtorrent_session_paused(&mut self) -> bool;

    pub rpc fn move_storage(&mut self, torrent_ids: &[InfoHash], dest: &str);

    #[rpc(method = "pause_session")]
    pub rpc fn pause_libtorrent_session(&mut self);

    pub rpc fn pause_torrent(&mut self, torrent_id: InfoHash);

    pub rpc fn pause_torrents(&mut self, torrent_ids: &[InfoHash]);

    // TODO: MagnetMetadata struct
    pub rpc fn prefetch_magnet_metadata<T: DeserializeOwned>(&mut self, magnet: &str, timeout: u64) -> (InfoHash, HashMap<String, T>);

    pub rpc fn queue_bottom(&mut self, torrent_ids: &[InfoHash]);

    pub rpc fn queue_down(&mut self, torrent_ids: &[InfoHash]);

    pub rpc fn queue_top(&mut self, torrent_ids: &[InfoHash]);

    pub rpc fn queue_up(&mut self, torrent_ids: &[InfoHash]);

    #[rpc(auth_level = "Admin")]
    pub rpc fn remove_account(&mut self, username: &str);

    pub rpc fn remove_torrent(&mut self, torrent_id: InfoHash, remove_data: bool);

    pub rpc fn remove_torrents(&mut self, torrent_ids: &[InfoHash], remove_data: bool);

    pub rpc fn rename_files(&mut self, torrent_id: InfoHash, filenames: &[(u64, &str)]);

    pub rpc fn rename_folder(&mut self, torrent_id: InfoHash, folder: &str, new_folder: &str);

    pub rpc fn rescan_plugins(&mut self);

    #[rpc(method = "resume_session")]
    pub rpc fn resume_libtorrent_session(&mut self);

    pub rpc fn resume_torrent(&mut self, torrent_id: InfoHash);

    pub rpc fn resume_torrents(&mut self, torrent_ids: &[InfoHash]);

    pub rpc fn set_config(&mut self, config: HashMap<String, impl Serialize>);

    pub rpc fn set_torrent_options(&mut self, torrent_ids: &[InfoHash], options: &TorrentOptions);

    pub rpc fn test_listen_port(&mut self) -> bool;

    #[rpc(auth_level = "Admin")]
    pub rpc fn update_account(&mut self, username: &str, password: &str, auth_level: AuthLevel);

    pub rpc fn upload_plugin(&mut self, filename: &str, filedump: &[u8]);
}

rpc_class! {
    impl Session::label;

    pub rpc fn get_labels(&mut self) -> Vec<String>;

    #[rpc(method = "add")]
    pub rpc fn add_label(&mut self, label_id: &str);

    #[rpc(method = "remove")]
    pub rpc fn remove_label(&mut self, label_id: &str);

    #[rpc(method = "get_options")]
    pub rpc fn get_label_options<T: DeserializeOwned>(&mut self, label_id: &str) -> HashMap<String, T>;

    #[rpc(method = "set_options")]
    pub rpc fn set_label_options(&mut self, label_id: &str, options: HashMap<String, impl Serialize>);

    #[rpc(method = "set_torrent")]
    pub rpc fn set_torrent_label(&mut self, torrent_id: InfoHash, label_id: &str);

    #[rpc(method = "get_config")]
    pub rpc fn get_label_config<T: DeserializeOwned>(&mut self) -> HashMap<String, T>;

    #[rpc(method = "set_config")]
    pub rpc fn set_label_config(&mut self, config: HashMap<String, impl Serialize>);
}