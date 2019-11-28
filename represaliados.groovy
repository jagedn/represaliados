@Grab(group='org.ccil.cowan.tagsoup',
      module='tagsoup', version='1.2' )
def tagsoupParser = new org.ccil.cowan.tagsoup.Parser()
def slurper = new XmlSlurper(tagsoupParser)

def html = "http://pares.mcu.es/victimasGCFPortal/detalle.form?idpersona=${args[0]}".toURL().getText('iso-8859-1')
def htmlParser = slurper.parseText(html)

String nombre 
String poblacion 
String residencia
String profesion
String tipologia 
htmlParser.'**'.findAll{ it.@summary == 'Detalle'}.each {
	nombre = "${it.tr[0].td.strong}".trim()
	poblacion = "${it.tr[2].td}".trim()
	residencia = "${it.tr[3].td}".trim()
	profesion= "${it.tr[4].td}".trim()
}
htmlParser.'**'.findAll{ it.@summary == 'Expediente'}.each {
	item = it.'**'.find{ "${it?.th}".startsWith('Tipol')  }
	tipologia = item ? item.td : null
}
println """
$nombre
$poblacion ($residencia)
de profesión $profesion
$tipologia

Represaliado num: ${args[0]}

Para saber
http://pares.mcu.es/victimasGCFPortal/detalle.form?idpersona=${args[0]}
vía @ArchivosEst
"""
