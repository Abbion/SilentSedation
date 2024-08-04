DATABASE_NAME = "user_data"

def add_card():
    cards = ""
    next_id = 0

    while True:
        should_add_card = input("Add card? [Y]/[N]: ").upper()

        if len(should_add_card) != 1 or not should_add_card.isalpha() or not any((option in "YN") for option in should_add_card):
            print(should_add_card + " is not an option!")
            continue
        
        should_add_card = should_add_card.upper()

        if should_add_card == "Y":
            device_name = input("Device name: ")

            if not device_name.isalnum():
                print("Device name not allowed! Use only alpha numeric values")
                continue

            print("Select type")
            print("1. Schock coller")

            device_type = input("Type: ")

            if len(device_type) != 1 or not device_type.isnumeric():
                print("Incorrect device type!")
                continue
            
            if int(device_type) < 1 or int(device_type) > 1:
                print("Device type out of range")
                continue

            if len(cards) > 0:
                cards += ", "

            cards += "{id: " + str(next_id) + ", device_type: " + device_type + ", device_name: \"" + device_name + "\"}"
            next_id += 1

        else:
            return cards


def main():
    print("Silent sedation db user creation tool")

    username = ""

    while True:
        username = input("Username: ")

        if ' ' in username:
            print("Username cannot have white sapces!");
            continue
    
        if len(username) < 6 or len(username) > 32:
            print("Username is too short or too long. Username length form 6 to 32 characters")
            continue

        break

    password = ""

    while True:
        password = input("Password: ")

        if ' ' in password:
            print("Password cannot have white spaces!")
            continue

        if len(password) < 8 or len(password) > 64:
            print("Password is too short or too long. Password length from 8 to 64 characters")
            continue

        break
    
    cards = add_card()

    print("db." + DATABASE_NAME + ".insertOne({username:\"" + username + "\"" + ", password:\"" + password + "\"" + ", cards:[" + cards + "]})")

if __name__ == "__main__":
    main()
