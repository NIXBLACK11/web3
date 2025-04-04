trait Shape {
    fn area(&self) -> u32;
  }
  
  struct Rect {
    widht: u32,
    height: u32
  }
  
  impl Shape for Rect {
    fn area(&self) -> u32 {
      return self.widht * self.height
    }
  }
  
  fn get_area1<T: Shape>(shape: &T) -> u32 {
    return shape.area();
  }
  
  // Or I can use the following syntax
  fn get_area2(shape: impl Shape) -> u32 {
    return shape.area();
  }
  
  fn main() {
    print!("hello world!");
    let r: Rect = Rect { widht: 10, height: 10 };
  
    get_area1(&r);
  }