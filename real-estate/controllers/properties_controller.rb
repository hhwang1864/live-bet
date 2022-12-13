get '/' do
  properties = all_properties()

  erb :'properties/index', locals: {
    properties: properties 
  }
end

get '/properties/new' do
 
  if !logged_in?
    redirect'/'
  end

  erb :'properties/new'
end

post '/properties' do

  if !logged_in?
    redirect'/'
  end

  name = params['name']
  image_url = params['image_url']
  region = params['region']
  bedroom_no = params['bedroom_no']
  price = params['price']

  create_property(name, image_url, region, bedroom_no, price)
 
  redirect '/'
end

get '/properties/:id/edit' do

  if !logged_in?
    redirect'/'
  end

  id = params['id']
  property = get_property(id)
  
  erb :'properties/edit', locals: {
    property: property
  }
end

put '/properties:id' do

  if !logged_in?
    redirect'/'
  end

  id = params['id']
  name = params['name']
  image_url = params['image_url']
  region = params['region']
  bedroom_no = params['bedroom_no']
  price = params['price']

  update_property(id, name, image_url, region, bedroom_no, price)
  # We ALWAYS redirect the suer at the end of a POST, PUT or DELETE method.
  redirect '/'
end

delete '/properties/:id' do

  if !logged_in?
    redirect'/'
  end
  
  id = params['id']
  delete_property(id)
  
  redirect '/'
end

