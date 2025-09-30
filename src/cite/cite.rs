/*
* page들을 등록하고 page 레이아웃을 방문자들을 통해 스캔하면서 메타데이터 수집 빌드 진행
*/
pub trait cite {
    fn build(&self);
    fn get_attr(&self);
    fn visitor(&self);
}

