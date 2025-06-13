# abv - Always Be Vaultin'

This is a set of scripts that will allow you to easily decrypt vaulted variables in an Ansible Vault file. It reads a configured `vault_identity_list` from `ansible.cfg` to decrypt variables without needing to manually specify a vault identity.

# Rust 🦀

_In progress._

# Ruby

## Requirements

- Ruby
- Ruby gem: `parseconfig`
- Ruby gem: `yaml`
- Ansible

## Setup

- Clone this repo locally.
- Configure the ABV config file (`~/.abv.cfg`) to point to your `ansible.cfg` of choice:
```
[ansible]
config_file_path = ~/path_to/ansible/ansible.cfg
```
- (optional) Add an alias for `abv.rb` (or whatever you'd like to more easily run the script):
```
alias agrep='ruby ~/path_to/ruby-abv/abv.rb'
```

## Usage

I like aliasing this to `agrep`, because that's probably a better name than "ABV". Whatever.

```
agrep vault_variable_name path_to/vault.yml
```

## Example

- `ansible.cfg` contains:
```
vault_identity_list = example_identity@~/.vault_password_example
```

- `~/.vault_password_example` contains:
```
PasswordsRFun
```

- `string_to_vault` contains:
```
this is super secret!!!
```

First, let's vault the above string and store it in a vault file:

```
$ cat string_to_vault|tr -d '\n'|ansible-vault encrypt_string --encrypt-vault-id example_identity
Reading plaintext input from stdin. (ctrl-d to end input)
!vault |
          $ANSIBLE_VAULT;1.2;AES256;example_identity
          61623732656565663338373136623366326238663161623637303863386432373934376633316136
          3864383965316137363936656132363635306231313836660a316535653034383831623337376662
          33616163386164333262623665313766646464396261313662353666366461613066633063623733
          6462323438386261660a653832393261663661663939313136323565376334323363393161646237
          32343561343035383835303330633363373833316131383539643863333232393234
```

Let's create `test_vault.yml` with the following:
```
vault_test_variable: !vault |
          $ANSIBLE_VAULT;1.2;AES256;example_identity
          61623732656565663338373136623366326238663161623637303863386432373934376633316136
          3864383965316137363936656132363635306231313836660a316535653034383831623337376662
          33616163386164333262623665313766646464396261313662353666366461613066633063623733
          6462323438386261660a653832393261663661663939313136323565376334323363393161646237
          32343561343035383835303330633363373833316131383539643863333232393234
```

Now, let's decrypt the above variable:

```
$ agrep vault_test_variable test_vault.yml
Decryption successful
this is super secret!!!
```
