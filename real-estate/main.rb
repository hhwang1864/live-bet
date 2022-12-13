require 'sinatra'
require 'bcrypt'
require './db/db'

enable :sessions

require './models/properties'
require './models/user'


require './controllers/properties_controller'
require './controllers/users_controller'
require './controllers/sessions_controller'

require './helpers/sessions_helper'