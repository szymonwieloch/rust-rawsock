use Device;
use Error;

pub struct PFRingDeviceIterator {

}

impl Iterator for PFRingDeviceIterator{
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
    type Item = Device;
}