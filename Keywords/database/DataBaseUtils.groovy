package database

import java.sql.Connection //added
import java.sql.DriverManager
import java.sql.ResultSet
import java.sql.Statement

import com.kms.katalon.core.annotation.Keyword
//import com.mysql.jdbc.Connection


public class DataBaseUtils {

	private static Connection connection = null;

	/**
	 * Open and return a connection to database
	 * @param dataFile absolute file path
	 * @return an instance of java.sql.Connection
	 */

	//Establishing a connection to the DataBase

	@Keyword

	def connectDB(String url, String dbname, String port, String username, String password){

		//Load driver class for your specific database type

		String conn = "jdbc:oracle:thin:@" + url + ":" + port + ":" + dbname
		//String conn = "jdbc:oracle:thin:@192.168.8.52:1521:orcl"

		Class.forName("oracle.jdbc.driver.OracleDriver")

		if(connection != null && !connection.isClosed()){

			connection.close()

		}

		connection = DriverManager.getConnection(conn, username, password)

		return connection

	}

	/**
	 * execute a SQL query on database
	 * @param queryString SQL query string
	 * @return a reference to returned data collection, an instance of java.sql.ResultSet
	 */

	//Executing the constructed Query and Saving results in resultset

	@Keyword

	def executeQuery(String queryString) {

		Statement stm = connection.createStatement()

		ResultSet rs = stm.executeQuery(queryString)

		return rs

	}


	/**
	 * execute a SQL "single column" query on database.
	 * the query should be constructed to return one column with a unique value
	 * e.g. select ID from qp_central.branch where name = 'Tokyo'
	 * @param queryString SQL query string
	 * @return a matched value from single column
	 */

	//Executing the constructed Query and returning result

	@Keyword

	def executeMatchQuery(String queryString, String match) {

		Statement stm = connection.createStatement()

		ResultSet rs = stm.executeQuery(queryString)

		def oid =""
		while (rs.next()) {
			oid = rs.getString(match)
		}
		return oid

	}


	//Closing the connection

	@Keyword

	def closeDatabaseConnection() {

		if(connection != null && !connection.isClosed()){

			connection.close()

		}

		connection = null

	}

	/**
	 * Execute non-query (usually INSERT/UPDATE/DELETE/COUNT/SUM...) on database
	 * @param queryString a SQL statement
	 * @return single value result of SQL statement
	 */

	@Keyword

	def execute(String queryString) {

		Statement stm = connection.createStatement()

		boolean result = stm.execute(queryString)

		return result

	}

}
