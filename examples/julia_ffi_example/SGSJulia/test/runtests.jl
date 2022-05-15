
using Test
import SGSJulia

@testset "client new->destroy" begin
    client = SGSJulia.SGSClient()
    SGSJulia.destroy!(client)
    @test client.handle == C_NULL
end

@testset "get_ultimo_valor_xml" begin
    client = SGSJulia.SGSClient()
    str = SGSJulia.get_ultimo_valor_xml(client, 1)
    @test startswith(str, "<?xml version")
end
