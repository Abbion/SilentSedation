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
            print("Select type")
            print("1. Schock coller")

            device_type = input("Type: ")

            if len(device_type) != 1 or not device_type.isnumeric():
                print("Incorrect device type!")
                continue
            
            device_type = int(device_type)

            if device_type < 1 or device_type > 1:
                print("Device type out of range")
                continue

            if len(cards) > 0:
                cards += ", "

            cards += "{id: Long(" + str(next_id) + "), device_type: " + str(device_type)
            
            if device_type != 0:
                device_name = input("Device name: ")

                if not device_name.isalnum():
                    print("Device name not allowed! Use only alpha numeric values")
                    continue

                cards += ", device_name: \"" + device_name + "\""
           
                if device_type == 1:
                    cards += ", power: 5"

            cards += ", code: \"123456\"}"
            next_id += 1

        else:
            #Add empty device
            cards += ", {id: Long(" + str(next_id) + "), device_type: 0, code: \"123456\"}"
            return (cards, next_id)


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
    
    cards, next_card_id = add_card()

    print("db." + DATABASE_NAME + ".insertOne({username:\"" + username + "\"" + ", password:\"" + password + "\"" + ", next_card_id: Long(" + str(next_card_id) + "), cards:[" + cards + "]})")

if __name__ == "__main__":
    main()
