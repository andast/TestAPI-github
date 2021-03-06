<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET search</name>
   <tag></tag>
   <elementGuidId>c2c72f42-fb7d-44f4-84bb-03439d1631f7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;firstName\&quot;: \&quot;${firstName}\&quot;,\n\t\&quot;lastName\&quot;: \&quot;${lastName}\&quot;,\n\t\&quot;dateOfBirth\&quot;: \&quot;${dateOfBirth}\&quot;,\n\t\&quot;phone\&quot;: \&quot;${phone}\&quot;,\n\t\&quot;email\&quot;: \&quot;${email}\&quot;,\n\t\&quot;identificationNumber\&quot;: \&quot;${identificationNumber}\&quot;,\n\t\&quot;externalId\&quot;: \&quot;${externalId}\&quot;,\n\t\&quot;addressLine1\&quot;: \&quot;${addressLine1}\&quot;,\n\t\&quot;addressLine2\&quot;: \&quot;${addressLine2}\&quot;,\n\t\&quot;addressZip\&quot;: \&quot;${addressZip}\&quot;,\n\t\&quot;addressCity\&quot;: \&quot;${addressCity}\&quot;,\n\t\&quot;addressState\&quot;: \&quot;${addressState}\&quot;,\n\t\&quot;addressCountry\&quot;: \&quot;${addressCountry}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Referer</name>
      <type>Main</type>
      <value>${orchestraHost}/</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${basicAuth}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${orchestraHost}/calendar-backend/api/v1/resourcegroups/search?${searchString}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.orchestraHost</defaultValue>
      <description></description>
      <id>43fbcc0b-47de-4dcb-9571-15021e62f62c</id>
      <masked>false</masked>
      <name>orchestraHost</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.basicAuth</defaultValue>
      <description></description>
      <id>a7d9146c-9d09-48d5-9149-efc732e61883</id>
      <masked>false</masked>
      <name>basicAuth</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>68ec9c53-ff22-4d72-b1ea-6859ea380b9e</id>
      <masked>false</masked>
      <name>searchString</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
