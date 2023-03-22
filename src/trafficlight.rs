enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义一个trait，包含一个返回时间的方法
trait LightDuration {
    fn duration(&self) -> u32;
}

impl LightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 20,    // 红灯持续20秒
            TrafficLight::Yellow => 5,  // 黄灯持续5秒
            TrafficLight::Green => 30,  // 绿灯持续30秒
        }
    }
}

fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    println!("红灯持续时间: {}秒", red_light.duration());
    println!("黄灯持续时间: {}秒", yellow_light.duration());
    println!("绿灯持续时间: {}秒", green_light.duration());
}

