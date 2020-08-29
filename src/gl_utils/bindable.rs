// TODO: Currently Bindable isn't really used for any dynamic dispatch
//       Evaluate if it should be removed
pub trait Bindable {
    fn bind(&self);
    fn unbind(&self);
}