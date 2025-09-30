/*
* block 들을 수집해 온전한 html 파일을 만드는 계층
* 한 파일이 그대로 한 html 파일이 된다
*/
pub trait Page {
    fn build(&self);
    fn accept(&self);
}

