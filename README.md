# sgs-bacen-rs

A SOAP client for Brazilian Central Bank's Time Series Management System <https://www3.bcb.gov.br/sgspub>.

# Usage

Running `cargo build` will create the binary `sgscli` that
can be used to query values from SGS.

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

```shell
$ sgscli series --list 1,2 --from 01/01/2020 --to 05/01/2020
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
        <SERIE ID='2'>
                <ITEM>
                        <DATA>2/1/2020</DATA>
                        <VALOR></VALOR>
                        <BLOQUEADO>false</BLOQUEADO>
                </ITEM>
                <ITEM>
                        <DATA>3/1/2020</DATA>
                        <VALOR></VALOR>
                        <BLOQUEADO>false</BLOQUEADO>
                </ITEM>
        </SERIE>
</SERIES>
```

# Resources

* WSDL: <https://www3.bcb.gov.br/sgspub/JSP/sgsgeral/FachadaWSSGS.wsdl>

* SGS Website: <https://www3.bcb.gov.br/sgspub/>

* SGS Docs: <https://www3.bcb.gov.br/sgspub/JSP/sgsgeral/sgsAjudaIng.jsp>