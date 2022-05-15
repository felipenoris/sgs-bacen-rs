
using Test
import SGS

@testset "client new->destroy" begin
    client = SGS.SGSClient()
    SGS.destroy!(client)
    @test client.handle == C_NULL
end

@testset "get_ultimo_valor_xml" begin
    client = SGS.SGSClient()
    str = SGS.get_ultimo_valor_xml(client, 1)
    @test startswith(str, "<?xml version")
end
