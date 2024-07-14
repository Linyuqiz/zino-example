mod router;

use zino::prelude::*;

fn main() {
    zino::Cluster::boot().register(router::routes()).run()
}
