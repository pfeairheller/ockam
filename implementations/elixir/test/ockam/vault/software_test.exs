defmodule Ockam.Vault.Software.Tests do
  use ExUnit.Case, async: true
  doctest Ockam.Vault.Software

  describe "Ockam.Vault.Software.sha256/2" do
    test "can run natively implemented functions" do
      handle = Ockam.Vault.Software.default_init
      hash = Ockam.Vault.Software.sha256(handle, "test")
      assert hash == <<159, 134, 208, 129, 136, 76, 125, 101, 154,
                       47, 234, 160, 197, 90, 208, 21, 163, 191,
                       79, 27, 43, 11, 130, 44, 209, 93, 108,
                       21, 176, 240, 10, 8>>
    end
  end
end
