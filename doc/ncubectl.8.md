NCUBECTL(8) - System Manager's Manual

# NAME

**ncubectl** - control the Ncube daemon

# SYNOPSIS

**ncubectl**
\[**-hV**&nbsp;**-d**&nbsp;*database*&nbsp;**-v**]
*command*
\[*argument&nbsp;...*]

# DESCRIPTION

The
**ncubectl**
program controls
ncubed(1).

The options are as follows:

\[**-d** *database*]

> Specify the path to the Ncube host database.

\[**-v**]

> Enable verbose logging. This argument can be specified multiple times to increase the log level.

\[**-h**]

> Print the usage message of
> **ncubectl**
> and exit.

\[**-V**]

> Print the version of
> **ncubectl**
> and exit.

The following commands are available:

**workspace** *name* \[*postgres\_url*]

> Bootstrap a new local workspace identified by
> *name*
> in the workspace root. The workspace slug is generated from the human readable
> *name*.

> As a default the workspace is bootstrapped with a SQLite
> database. Supply a
> *postgres\_url*
> to associate the workspace with a PostgreSQL database instead.

**account** *workspace* *email*

> Create a new account to grant remote access to
> *workspace*.
> The account uses
> *email*
> as an unique identifier for the account. A one-time password is generated for
> the account during creation.

**connection** *workspace* *email*

> Output the account details to connect to a workspace. The connection details
> are output as JSON in order to import them in the
> ncube(1)
> app.

**state** *modifier*

> Show the internal state of
> ncubed(1)
> specified by
> *modifier*:

> *workspaces*

> > List all workspaces configured for
> > ncubed(1).

> *accounts*

> > List all accounts that are associated to a workspace.

> *all*

> > Show all states for
> > ncubed(1).

**get**

> Show all configuration settings for
> ncubed(1).

**set** *setting* *value*

> Set a configuration setting for
> ncubed(1).

> *setting*

> > Specify the name of the setting to set.

> *value*

> > Provide a new value for this setting. If this filed is omitted the setting is
> > unset instead.

**reset** *modifier*

> Reset a configuration setting.
> *modifier*:

> *secret*

> > Regenerate the secret key of
> > ncubed(1).
> > The key is used to sign JWT tokens when accounts log in. The initial generation
> > happens during bootstrap. Server only installations of
> > ncubed(1)
> > have to run this command once right after initial installation.

**delete** **workspace** *workspace* \[**-y**]

> Delete a workspace and all it's associated accounts. This only deletes the
> workspace in
> ncubed(1).
> The
> sugarcube(1)
> project and any downloaded data must be deleted manually.

> **-y**

> > Automatic yes to prompts; assume &#34;yes&#34; as answer to all prompts and run
> > non-interactively.

**delete** **account** *workspace* *email* \[**-y**]

> Delete an account for
> *email*
> that is associated to the workspace
> *workspace*.

> **-y**

> > Automatic yes to prompts; assume &#34;yes&#34; as answer to all prompts and run
> > non-interactively.

# EXIT STATUS

The **ncubectl** utility exits&#160;0 on success, and&#160;&gt;0 if an error occurs.

# EXAMPLES

Create a new workspace for the local
ncubed(1):

	# ncubectl workspace "Syrian Archive"

Once the workspace is created accounts can be added for remote access. This will
generate a one-time password that can be used by the account holder to finish
account setup.

	# ncubectl account christo@syrianarchive.org syrian-archive
	# ncubectl connection christo@syrianarchive.org syrian-archive

# SEE ALSO

ncubed(1),
ncube(1),
sugarcube(1)

Mac OS X 10.15 - May 27, 2020
