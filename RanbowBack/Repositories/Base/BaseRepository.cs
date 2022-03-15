using MySql.Data.MySqlClient;
using System;
using System.Diagnostics;

namespace RanbowBack.Repositories.Base
{
    public class BaseRepository : IDisposable
    {
        protected MySqlConnection DbConnection { get; set; }
        public MySqlCommand DbCommand { get; set; }

        protected string ConnectionString { get; set; }

        public BaseRepository(string connectionString)
        {
            ConnectionString = connectionString;
            DbConnection = new MySqlConnection(connectionString);
            DbCommand = new MySqlCommand("", DbConnection);
        }

        public bool Check()
        {
            bool success = false;
            try
            {
                DbConnection.Open();
                DbConnection.Close();

                success = true;
            }
            catch (Exception e)
            {
                //TODO: Add logging
                Debug.WriteLine(e.Message);
            }
            finally
            {
                if (DbConnection.State == System.Data.ConnectionState.Open)
                    DbConnection.Close();
            }

            return success;
        }

        public void Dispose()
        {
            DbCommand.Dispose();
            DbConnection.Dispose();
            ConnectionString = null;

            GC.SuppressFinalize(this);
        }
    }
}
