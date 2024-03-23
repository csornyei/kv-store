use super::key::Key;

#[cfg(test)]
mod new_key {
    use super::Key;
    #[test]
    fn test_key_new_default_store() {
        let key = Key::new(".".to_string());

        assert_eq!(key.store, Some(".".to_string()));
        assert_eq!(key.path, None);
        assert_eq!(key.key, None);
    }

    #[test]
    fn test_key_new_no_store() {
        let key = Key::new("key".to_string());

        assert_eq!(key.store, None);
        assert_eq!(key.path, None);
        assert_eq!(key.key, Some("key".to_string()));
    }

    #[test]
    fn test_key_new_with_store() {
        let key = Key::new("store:key".to_string());

        assert_eq!(key.store, Some("store".to_string()));
        assert_eq!(key.path, None);
        assert_eq!(key.key, Some("key".to_string()));
    }

    #[test]
    fn test_key_new_with_path() {
        let key = Key::new("default_store:second_store:third_store:key".to_string());

        assert_eq!(key.store, Some("default_store".to_string()));
        assert_eq!(key.path, Some("second_store:third_store".to_string()));
        assert_eq!(key.key, Some("key".to_string()));
    }
}

#[cfg(test)]
mod key_to_str {
    use super::Key;

    #[test]
    fn test_key_to_str_default_store() {
        let key = Key {
            store: Some(".".to_string()),
            path: None,
            key: None,
        };

        assert_eq!(key.to_str(), ".".to_string());
    }

    #[test]
    fn test_key_to_str_no_store() {
        let key = Key {
            store: None,
            path: None,
            key: Some("key".to_string()),
        };

        assert_eq!(key.to_str(), "key".to_string());
    }

    #[test]
    fn test_key_to_str_with_store() {
        let key = Key {
            store: Some("store".to_string()),
            path: None,
            key: Some("key".to_string()),
        };

        assert_eq!(key.to_str(), "store:key".to_string());
    }

    #[test]
    fn test_key_to_str_with_path() {
        let key = Key {
            store: Some("default_store".to_string()),
            path: Some("second_store:third_store".to_string()),
            key: Some("key".to_string()),
        };

        assert_eq!(
            key.to_str(),
            "default_store:second_store:third_store:key".to_string()
        );
    }
}

#[cfg(test)]
mod key_get_next_key {
    use super::Key;

    #[test]
    fn test_key_get_next_key_no_store() {
        let key = Key {
            store: None,
            path: None,
            key: Some("key".to_string()),
        };

        let next_key = key.get_next_key();

        assert_eq!(next_key.store, None);
        assert_eq!(next_key.path, None);
        assert_eq!(next_key.key, Some("key".to_string()));
    }

    #[test]
    fn test_key_get_next_key_with_no_path() {
        let key = Key {
            store: Some("store".to_string()),
            path: None,
            key: Some("key".to_string()),
        };

        let next_key = key.get_next_key();

        assert_eq!(next_key.store, None);
        assert_eq!(next_key.path, None);
        assert_eq!(next_key.key, Some("key".to_string()));
    }

    #[test]
    fn test_key_get_next_key_with_single_element_path() {
        let key = Key {
            store: Some("store".to_string()),
            path: Some("path".to_string()),
            key: Some("key".to_string()),
        };

        let next_key = key.get_next_key();

        assert_eq!(next_key.store, Some("path".to_string()));
        assert_eq!(next_key.path, None);
        assert_eq!(next_key.key, Some("key".to_string()));
    }

    #[test]
    fn test_key_get_next_key_with_multiple_element_path() {
        let key = Key {
            store: Some("store".to_string()),
            path: Some("path1:path2".to_string()),
            key: Some("key".to_string()),
        };

        let next_key = key.get_next_key();

        assert_eq!(next_key.store, Some("path1".to_string()));
        assert_eq!(next_key.path, Some("path2".to_string()));
        assert_eq!(next_key.key, Some("key".to_string()));
    }
}

#[cfg(test)]
mod key_valid_get_key {
    use super::Key;

    #[test]
    fn test_key_valid_get_key_no_key() {
        let key = Key {
            store: Some("store".to_string()),
            path: Some("path".to_string()),
            key: None,
        };

        assert_eq!(key.valid_get_key(), false);
    }

    #[test]
    fn test_key_valid_get_key_with_key() {
        let key = Key {
            store: Some("store".to_string()),
            path: Some("path".to_string()),
            key: Some("key".to_string()),
        };

        assert_eq!(key.valid_get_key(), true);
    }

    #[test]
    fn test_key_valid_get_key_default_store() {
        let key = Key::new(".".to_string());

        assert_eq!(key.valid_get_key(), false);
    }

    #[test]
    fn test_key_valid_get_key_no_store() {
        let key = Key::new("key".to_string());

        assert_eq!(key.valid_get_key(), true);
    }

    #[test]
    fn test_key_valid_get_key_with_store() {
        let key = Key::new("store:key".to_string());

        assert_eq!(key.valid_get_key(), true);
    }

    #[test]
    fn test_key_valid_get_key_with_path() {
        let key = Key::new("default_store:second_store:key".to_string());

        assert_eq!(key.valid_get_key(), true);
    }
}
