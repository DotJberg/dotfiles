return {
  "nvimtools/none-ls.nvim",
  dependencies = {
    "nvimtools/none-ls-extras.nvim",
    "jay-babu/mason-null-ls.nvim", -- bridges gap between mason & null-ls
  },
  config = function()
    require("mason-null-ls").setup({
      ensure_installed = {
        -- Opt to list sources here, or leave empty to use what's installed
        "stylua",
        "gofumpt",
        "golangci_lint",
        "biome",
        "eslint_d"
      },
      automatic_installation = false,
      handlers = {}, -- Empty table means "use default handler" (setup automatic sources)
    })

    require("null-ls").setup({
      sources = {
        -- Anything not supported by mason-null-ls can go here
      },
    })

    vim.keymap.set("n", "<leader>gf", vim.lsp.buf.format, {})
  end,
}
