mod hash;
mod tree;
mod vector;

use hash::List as HList;
use tree::List as TList;
use vector::List as VList;

fn main() {
    let mut vlist = VList::new();

    vlist.add("Name01", "Content01", true);
    vlist.add("Name02", "Content02", false);
    vlist.add("Name", "Content", false);

    vlist.update(3, "Name03", "Content03", true);
    vlist.remove(2);

    print!("\n{}\n\n", "Vector Collection:");

    print!(
        "\n{:#?}, {:#?}, {:#?}\n\n",
        vlist,
        vlist.all(),
        vlist.one(3)
    );

    let mut hlist = HList::new();

    hlist.add("Name01", "Content01", true);
    hlist.add("Name02", "Content02", false);
    hlist.add("Name", "Content", false);

    hlist.update(3, "Name03", "Content03", false);
    hlist.remove(2);

    print!("\n{}\n\n", "HashMap Collection");

    print!(
        "\n{:#?}, {:#?}, {:#?}\n\n",
        hlist,
        hlist.all(),
        hlist.one(3)
    );

    let mut tlist = TList::new();

    let item1 = tlist
        .add("Name01", "Content01", true)
        .expect("Adding item 1 failed")
        .clone();
    let item2 = tlist
        .add("Name02", "Content02", false)
        .expect("Adding item 2 failed")
        .clone();
    let item3 = tlist
        .add("Name", "Content", false)
        .expect("Adding item 3 failed")
        .clone();

    tlist.update(item3.id(), "Name03", "Content03", true);
    tlist.remove(item2.id());

    print!("\n{}\n\n", "BTreeMap Collection:");

    print!(
        "\n{}, {}, {}, {}\n\n",
        item1.id(),
        item1.name(),
        item1.content(),
        &item1.active().to_string(),
    );

    print!(
        "\n{:#?}, {:#?}, {:#?}\n\n",
        tlist,
        tlist.all(),
        tlist.one(item3.id())
    );
}
