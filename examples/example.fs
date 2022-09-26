
if 10 != 5 {
    print("10 is not equal to 5")
} else if 10 == 5 {
    print("10 is equal to 5")
} else {
    print("10 is not equal to 5")
}

const foo = () => {
    return 10
}

match foo() {
    10 => print("foo is 10")
    5 => print("foo() returns 5")
    _ => print("foo() returns something else")
}

for i in 0..5 {
    print(i)
}

const json = {
    foo: 10
    bar: 5
}


const xml = <div></div>

const bar = (
    name string,
    age int
) => {
    print("Hello " + name)
}

bar(name = "John", age = 10)

const fetch_something = async () => {

}

const data = await(fetch_something)

const people = [
    {
        id: 1
        name: "mikko"
    },
    {
        id: 2
        name: "john"
    }
]

const sql = select id, name from people where id == 1

for row in sql {
    print(row.id)
    print(row.name)
}

// struct Human {
//     name string = "qwer"
//     age int = 10
//     favorite_color string?
// }

Human.constructed = () => {
    print("{} was constructed", this.name)
}

Human.destroyed = () => {
    print("{} was destroyed", this.name)
}

Human.greet = () => {
    print("Hello, my name is {} and I'm {} years old", this.name, this.age)
}


const new_human = Human { }

new_human.greet()

type Reader {
    read: () => string
}

type Writer {
    write: () => void
}

mod io {
    export const copy = (src: Reader, dst: Writer) => {
        print("copy")
    }
}

use io::copy

enum Status {
    Ok(Human),
    Error {
        code: int
        message: string
    }
}

type Person {
    name string @db(type=varchar(60))
    age int @db(type=int)
    ballBasket @db(name=ball_basket)
}