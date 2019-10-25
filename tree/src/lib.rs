mod binary_search_tree;
mod btree;
mod heap;
mod red_black_tree;
mod trie;


#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub path: String,
    pub address: String,
}

impl IoTDevice {
    pub fn new(id: u64, address: impl Into<String>, path: impl Into<String>) -> IoTDevice {
        IoTDevice {
            address: address.into(),
            numerical_id: id,
            path: path.into(),
        }
    }
}

impl PartialEq for IoTDevice {
    fn eq(&self, other: &IoTDevice) -> bool {
        self.numerical_id == other.numerical_id && self.address == other.address
    }
}

#[derive(Clone, Debug)]
pub struct MessageNotification {
    pub no_messages: u64,
    pub device: IoTDevice,
}

impl MessageNotification {
    pub fn new(device: IoTDevice, no_messages: u64) -> MessageNotification {
        MessageNotification{
            no_messages: no_messages,
            device: device,
        }
    }
}

impl PartialEq for MessageNotification {
    fn eq(&self, other: &MessageNotification) -> bool {
        self.device.eq(&other.device) && self.no_messages == other.no_messages
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use crate::*;
    use rand::thread_rng;
    use rand::Rng;
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use test::Bencher;

    #[bench]
    fn bench_unsorted_insert_rbt_find(b: &mut Bencher) {
        let mut tree = red_black_tree::BetterDeviceRegistry::new_empty();
        let mut items: Vec<IoTDevice> = (0..LIST_ITEMS).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items {
            tree.add(item);
        }

        assert_eq!(tree.length, LIST_ITEMS);

        b.iter(|| {
            let r = rng.gen_range::<u64>(0, LIST_ITEMS);
            tree.find(r).expect("NOT FOUND")
        });
    }
}