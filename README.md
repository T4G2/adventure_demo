# adventure_demo

This project is created as free-time project

## Quick anatomy of .av file

Every .av file is separated in sections:

Sections starts from second line in format `SECTION_NAME:` and ends with `---`

### Section Types

For now there are 3 main section types:

#### VARS
  variable section type when you can define variable used in adventure like gold, money etc..
  
#### ITEMS
  items section are used for defining items, which can be picked and held in inventory
  
#### SCENE 
  this section defines one scene:
  it defines it's id, name , command sequence which executes at the start of scene and options
  
  options has their own attributes named, text and run 
  
  In scene there are two types of atributes, One Liners adn Multi Liners

One liners are only on one line in format `attr: value`

Multiliners must start with `attr:` then multiple lines of text and then they end by empty line ` `

for more questions and commands look at adventure_demo.av

