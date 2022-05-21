fn main() {}

#[cfg(test)]
mod test {
    use rspotify::model::PlaylistId;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Sample {
        id: PlaylistId,
    }

    static DATA: &[u8] = b"{\"id\": \"averyrealplaylistid\"}";

    #[test]
    fn reader() {
        let sample: Sample = serde_json::from_reader(DATA).unwrap();

        assert_eq!(sample.id, "averyrealplaylistid".parse().unwrap());
    }

    #[test]
    fn slice() {
        let sample: Sample = serde_json::from_slice(DATA).unwrap();

        assert_eq!(sample.id, "averyrealplaylistid".parse().unwrap());
    }
}
