enum Item {
    Heavy,
    Light,
}

mod container {
    use super::*;

    pub struct Container<const N: usize> {
        contents: [Option<Item>; N],
    }

    impl<const N: usize> Container<N> {
        pub fn drop(&mut self) -> Option<Item> {
            for item in self.contents.iter_mut() {
                if item.is_some() {
                    return item.take();
                }
            }
            None
        }

        pub fn load(&mut self, new_item: Item) -> Result<(), ()> {
            for item in self.contents.iter_mut() {
                if item.is_none() {
                    *item = Some(new_item);
                    return Ok(());
                }
            }
            Err(())
        }
    }
}
use container::Container;

type Dropper = Container<9>;

fn main() {

}
