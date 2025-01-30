require("config.lazy")

vim.keymap.set('n', '<left>', '<cmd>echo "Use h to move!!"<CR>')
vim.keymap.set('n', '<right>', '<cmd>echo "Use l to move!!"<CR>')
vim.keymap.set('n', '<up>', '<cmd>echo "Use j to move!!"<CR>')
vim.keymap.set('n', '<down>', '<cmd>echo "Use k to move!!"<CR>')

vim.opt.number = true

-- Enable break indent
vim.opt.breakindent = true

-- Tab width
vim.opt.shiftwidth = 4

-- Save undo history
vim.opt.undofile = true

-- Keep sign column on by default
vim.opt.signcolumn = 'yes'

-- Decrease update time
vim.opt.updatetime = 250

vim.opt.scrolloff = 10
vim.opt.cursorline = true
vim.keymap.set("n", "<Esc>", "<cmd>nohlsearch<CR>")

