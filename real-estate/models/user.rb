def create_user(first_name, last_name, email, password)
  password_digest = BCrypt::Password.create(password)
  # we create password -> brycpt password create function works then 
  run_sql("INSERT INTO users(first_name, last_name, email, password_digest) VALUES ($1, $2, $3, $4)", [first_name, last_name, email, password_digest])
end

def find_user_by_email(email)
  users = run_sql('SELECT * FROM users WHERE email = $1', [email])

  # if more than one user is found with that email address, pick the first one with that email. Otherwise if no one is found, return nil.
  if users.to_a.count > 0
    users[0]
  else
    nil
  end
end

  # we put [0] because the output will be array of users, but we only want 1 person. therefore we put [0]
def find_user_by_id(id)
  run_sql('SELECT * FROM users WHERE id = $1', [id])[0]
end