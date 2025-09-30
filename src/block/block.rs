/*  
* 의미론적 계층
* html element를 조합해 code,math 등 큰 단위 element를작성한다.
* 모든 block은 page에 속한다.
*/
pub trait Block {
    fn get_attr(&self);
    fn get_chids(&self);
    fn accept(&self);
    fn build(&self);
}
