use com::{interfaces, interfaces::iunknown::IUnknown};
use com::IID;

interfaces! {
    #[uuid(IID {
        data1: 0xC5F45CBC,
        data2: 0x4439,
        data3: 0x418C,
        data4: [0xA9, 0xF9, 0x05, 0xAC, 0x67, 0x52, 0x5E, 0x43],
    })]
    pub unsafe interface IExample: IUnknown {}
}
