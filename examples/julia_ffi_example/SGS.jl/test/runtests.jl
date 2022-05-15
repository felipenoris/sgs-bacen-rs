
using Test, Dates
import SGS

@testset "client new->destroy" begin
    client = SGS.SGSClient()
    SGS.destroy!(client)
    @test client.handle == C_NULL
end

client = SGS.SGSClient()

@testset "get_ultimo_valor_xml" begin
    str = SGS.get_ultimo_valor_xml(client, 1)
    @test startswith(str, "<?xml version")
end

@testset "get_valores_series_xml" begin
    str = SGS.get_valores_series_xml(client, [1, 12], Date(2022, 5, 1), Date(2022, 5, 5))
    @test startswith(str, "<?xml version")
end
