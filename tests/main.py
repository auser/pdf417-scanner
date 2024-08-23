from pdf417gen import encode, render_image, render_svg
from pydantic import BaseModel
import json


class Person(BaseModel):
    first_name: str
    last_name: str
    street_address: str
    city: str
    state: str
    zip_code: str
    phone_number: str
    email: str


def generate_barcode(text, num: int = 0):
    # Convert to code words
    codes = encode(text)

    # Generate barcode as image
    return render_image(codes)  # Pillow Image object


def generate_fake_person():
    from faker import Faker
    from faker.providers import address, person

    fake = Faker()
    fake.add_provider(address)
    fake.add_provider(person)

    person = Person(
        first_name=fake.first_name(),
        last_name=fake.last_name(),
        street_address=fake.street_address(),
        city=fake.city(),
        state=fake.state(),
        zip_code=fake.zipcode(),
        phone_number=fake.phone_number(),
        email=fake.email(),
    )

    return person.model_dump_json()


def main():
    from argparse import ArgumentParser

    parser = ArgumentParser()
    parser.add_argument("--num", type=int, default=1)
    parser.add_argument("--dir", default="fixtures/")
    args = parser.parse_args()

    people = []
    for i in range(args.num):
        fake_person_json = generate_fake_person()
        image = generate_barcode(fake_person_json, i)
        image.save(f"{args.dir}/barcode_{i}.jpg")
        people.append(fake_person_json)

    with open(f"{args.dir}/people.json", "w") as f:
        f.write(json.dumps(people))


if __name__ == "__main__":
    main()
