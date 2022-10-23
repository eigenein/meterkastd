#[derive(Debug)]
pub enum EnergyType {
    Gas,
    Electricity,
}

#[derive(Debug)]
pub enum FlowDirection {
    Consumption,
    Production,
}

#[derive(Debug)]
pub enum CounterType {
    Current,
    Cumulative,
}
