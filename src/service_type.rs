use crate::basic::*;

#[derive(Debug)]
struct ServiceType {
}

#[derive(Debug)]
struct MessageRefSet {
    message_ref:    Vec<MessageRefType>,
}

#[derive(Debug)]
struct ContainerRefSet {
    container_ref:  Vec<ContainerRefType>,
}
