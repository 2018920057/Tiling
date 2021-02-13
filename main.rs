use std::io;
use std::collections::HashMap;
//2*n 크기의 사각형을 2*1 크기의 타일로 채우는 방법의 수를 계산
fn main() {
  let mut input:String = String::new();
  println!("사각형의 가로 길이:");
  io::stdin().read_line(&mut input).expect("input error");
  let width: i32 = input.trim().parse().expect("parsing error");

  let mut cache: HashMap<i32,i32> = HashMap::new();
  println!("2*{}의 사각형을 2*1의 타일로 채우는 방법은 {}가지 입니다.",width,tiling(width, &mut cache));
}

fn tiling(width: i32, cache: &mut HashMap<i32,i32>)-> i32{
  //기저 사례: 가로길이가 2,0,1일 때
  if width==2 {return 2;}
  if width==1 {return 1;}
  if width<=0 {return 0;}
  //memoization:
  let ret :i32 = match cache.get(&width){
    Some(_) => 0,
    None => tiling(width-1,cache)+tiling(width-2,cache),
  };
  *cache.entry(width).or_insert(ret)
}