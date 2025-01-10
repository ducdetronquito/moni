# Moni


project list

project add --name "Vacances"
--> return project id: QwuICYInEFi

user list --project QwuICYInEFi

user add --project QwuICYInEFi --name "Guillaume"

user add --project QwuICYInEFi --name "Manon"


expense list --project QwuICYInEFi

expense add --project QwuICYInEFi --name "Courses" --date MM/DD/YYYY --made-by "Guillaume" --amount 30

expense add --project QwuICYInEFi --name "Essence" --date MM/DD/YYYY --made-by "Manon" --amount 20


balance --project QwuICYInEFi
