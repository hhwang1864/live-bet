def all_properties
  run_sql('SELECT * FROM properties ORDER BY id') 
end

def create_property(name, image_url, region, bedroom_no, price)
  run_sql("INSERT INTO properties(name, image_url, region, bedroom_no, price)
  VALUES($1, $2, $3, $4, $5)",[name, image_url, region, bedroom_no, price])
end

def get_property(id)
  run_sql('SELECT * FROM properties WHERE id = $1',[id])[0]
end

def update_property(id, name, image_url, region, bedroom_no, price)
  run_sql('UPDATE properties SET name = $2, image_url =$3, region = $4, bedroom_no = $5, price = $6 WHERE id = $1', [id, name, image_url, region, bedroom_no, price])
end

def delete_property(id)
  run_sql('DELETE FROM properties WHERE id = $1',[id])
end


def one_bedroom
  run_sql('SELECT * FROM properties WHERE bedroom_no = 1 ORDER BY id')
end

def two_bedroom
  run_sql('SELECT * FROM properties WHERE bedroom_no = 2 ORDER BY id')
end

def three_bedroom
  run_sql('SELECT * FROM properties WHERE bedroom_no >= 3 ORDER BY id')
end


