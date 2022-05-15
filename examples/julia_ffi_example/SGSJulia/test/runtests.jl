
using Test
import SGSJulia

@testset "get_ultimo_valor_xml" begin
	str = SGSJulia.get_ultimo_valor_xml(1)
	@test startswith(str, "<?xml version")
end
