@Grab(group='org.ccil.cowan.tagsoup', module='tagsoup', version='1.2' )
@Grab(group='org.twitter4j', module='twitter4j-core', version='4.0.6')

import twitter4j.TwitterFactory
import twitter4j.StatusUpdate
Random rnd = new Random()
first = args[0] as int
def tagsoupParser = new org.ccil.cowan.tagsoup.Parser()
def slurper = new XmlSlurper(tagsoupParser)
	
while( first < 1600000 ){	
	println first
	String html
	for(int i=0; i<3;i++){
		try{
			html= "http://pares.mcu.es/victimasGCFPortal/detalle.form?idpersona=${first}".toURL().getText('iso-8859-1')
			break
		}catch(e){
			sleep 1000*2
		}
	}
	if( !html ){
		first++;
		continue;
	}
	def htmlParser = slurper.parseText(html)

	String nombre 
	String poblacion 
	String residencia
	String profesion
	String tipologia 
	String expediente

	htmlParser.'**'.findAll{ it.@summary == 'Detalle'}.each {
		nombre = "${it.tr[0].td.strong}".trim()
		poblacion = "${it.tr[2].td}".trim()
		residencia = "${it.tr[3].td}".trim()
		profesion= "${it.tr[4].td}".trim()
	}
	htmlParser.'**'.findAll{ it.@summary == 'Expediente'}.each {
		item = it.'**'.find{ "${it?.th}".startsWith('Tipol')  }
		tipologia = item ? item.td : null
		item = it.'**'.find{ "${it?.th}".startsWith('Fecha de expediente')  }
		expediente = item ? item.td : null
	}
	if( nombre ){
		String message = """
		$nombre
		$poblacion ${residencia ? ','+residencia : ''}
		${ profesion ? 'Profesión '+profesion : ''}

		Fecha de expediente $expediente
		-$tipologia
		""".take(200)+"""

		Para saber sobre $nombre, visita:
		http://pares.mcu.es/victimasGCFPortal/detalle.form?idpersona=${first}	
		
		#represaliados 
		"""

		StatusUpdate status = new StatusUpdate(message)
		TwitterFactory.singleton.updateStatus status
		
		sleep 36000+(rnd.nextInt(30)*1000)
		if( (first%100) == 0 ) sleep 1000*60*(3+(rnd.nextInt(4)))
		if( (first%850) == 0 ) sleep 1000*60*(3+(rnd.nextInt(4)))
	}
	first++
}
