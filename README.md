# sgs-bacen-rs

A SOAP client for Brazilian Central Bank's Time Series Management System <https://www3.bcb.gov.br/sgspub>.

# Installation

Clone this repo and run `cargo install` passing the path to the `sgscli` subfolder.

```shell
git clone https://github.com/felipenoris/sgs-bacen-rs
cargo install --path ./sgs-bacen-rs/sgscli
```

After installation, the binary `sgscli` will be ready to use.

# Usage

The following command will print the last value
for the time series identified by code `1`.

```shell
$ sgscli last-value 1
<?xml version='1.0' encoding='ISO-8859-1'?>
<resposta status='2' descricao='Processado com sucesso'>
<SERIE>
        <NOME>Taxa de câmbio - Livre - Dólar americano (venda) - diário</NOME>
        <CODIGO>1</CODIGO>
        <PERIODICIDADE>D</PERIODICIDADE>
        <UNIDADE>u.m.c./US$</UNIDADE>
        <DATA>
                <DIA>22</DIA>
                <MES>4</MES>
                <ANO>2022</ANO>
        </DATA>
        <VALOR>4,7326</VALOR>
</SERIE>
</resposta>
```

The following example will print the historical values
for time series identified by codes `1` (PTAX USD) and `4389` (CDI a.a.)
from date `2020-01-01` to `2020-01-05`.

Dates can be informed using `yyyy-mm-dd` or `dd/mm/yyyy` format.

```shell
$ sgscli series 1 4389 --from 2020-01-01 --to 2020-01-05
<?xml version='1.0' encoding='ISO-8859-1'?>
<SERIES>
<SERIE ID='1'>
                <ITEM>
                        <DATA>2/1/2020</DATA>
                        <VALOR>4.0213</VALOR>
                        <BLOQUEADO>false</BLOQUEADO>
                </ITEM>
                <ITEM>
                        <DATA>3/1/2020</DATA>
                        <VALOR>4.0522</VALOR>
                        <BLOQUEADO>false</BLOQUEADO>
                </ITEM>
        </SERIE>
        <SERIE ID='4389'>
                <ITEM>
                        <DATA>2/1/2020</DATA>
                        <VALOR>4.40</VALOR>
                        <BLOQUEADO>false</BLOQUEADO>
                </ITEM>
                <ITEM>
                        <DATA>3/1/2020</DATA>
                        <VALOR>4.40</VALOR>
                        <BLOQUEADO>false</BLOQUEADO>
                </ITEM>
        </SERIE>
</SERIES>
```

# Examples

Check the `examples` folder for the following examples:

* `rust_example`: a Rust app that uses the `sgslib` Rust library.

* `c_example`: a C app that uses the FFI interface for `sgslib` Rust library.

* `julia_example`: a Julia library `SGS.jl` that uses the FFI interface for `sgslib` Rust library.

# Resources

* WSDL: <https://www3.bcb.gov.br/sgspub/JSP/sgsgeral/FachadaWSSGS.wsdl>

* SGS Website: <https://www3.bcb.gov.br/sgspub/>

* SGS Docs: <https://www3.bcb.gov.br/sgspub/JSP/sgsgeral/sgsAjudaIng.jsp>